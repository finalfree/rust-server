use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

pub struct Worker {
    work_thread: Option<thread::JoinHandle<()>>,
    id: usize,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));

        for x in 0..size {
            let worker = Worker::new(x, Arc::clone(&receiver));
            workers.push(worker);
        }

        ThreadPool {
            sender: Some(tx),
            workers,
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.work_thread.take() {
                thread.join().unwrap();
            }
            println!("Shutting down worker {}", worker.id);
        }
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let work_thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { work_thread: Some(work_thread), id }
    }
}