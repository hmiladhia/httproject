use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        match Self::build(id, receiver) {
            Ok(w) => w,
            Err(msg) => panic!("{}", &msg),
        }
    }

    fn build(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Result<Self, std::io::Error> {
        let thread = std::thread::Builder::new().spawn(move || loop {
            let job: Job = receiver.lock().unwrap().recv().unwrap();
            job()
        })?;

        Ok(Self { id, thread })
    }
}

#[derive(Debug)]
pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();

        let mut threads = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(rx));

        for id in 0..size {
            threads.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            threads,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
