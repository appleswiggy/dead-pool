use std::sync::{mpsc, Arc, Mutex};

use crate::worker::Worker;

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Creates a new thread pool with given number of threads.
    ///
    /// * Sets up a communication channel, and sends the receiver end to every thread.
    /// * Jobs will be sent to threads via this communication channel.
    pub fn new(size: usize) -> Self {
        assert!(size != 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    /// Sends a Job to the thread pool.
    ///
    /// * Any one free thread picks up the Job and executes it.
    ///
    /// # Examples
    ///
    /// ```
    /// let pool = ThreadPool::new(4);
    /// for _ in 0..100 {
    ///     pool.execute(|| {println!("Working");});
    /// }
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
