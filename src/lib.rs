use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, f: F) -> JoinHandle<()>
    where F: FnOnce() + Send + 'static
    {
        thread::spawn(|| {
            f();
        })
    }
}