use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
    work_thread: JoinHandle<()>,
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
            sender: tx,
            workers,
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let work_thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            job();
            println!("thread {} handled a request", id);
        });

        Worker { work_thread, id }
    }
}