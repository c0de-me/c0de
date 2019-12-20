use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

pub struct ThreadPool {
    wokers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut wokers = Vec::with_capacity(size);
        for id in 0..size {
            wokers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool {
            wokers,
            sender,
        }
    }

    pub fn exec<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job.call_box();
            }
        });
        Worker {
            id,
            thread,
        }
    }
}
