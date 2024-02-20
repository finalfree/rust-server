use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool{
    workers: Vec<JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(rx));

        for x in 0..size {
            let receiver_ref = Arc::clone(&receiver);
            let worker = thread::spawn(move || loop {
                let job = receiver_ref.lock().unwrap().recv().unwrap();

                job();
                println!("receive a job");
            });
            workers.push(worker);
        }

        ThreadPool {
            sender: tx,
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}