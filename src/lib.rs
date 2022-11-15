use std::{
  sync::{mpsc, Arc, Mutex},
  thread::{self},
};

struct Worker {
  id: usize,
  handle: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  threads: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut threads = Vec::with_capacity(size);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    for i in 0..size {
      threads.push(Worker::new(i, Arc::clone(&receiver)));
    }

    ThreadPool { threads, sender }
  }

  pub fn execute<F>(&self, func: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(func);

    self.sender.send(job).unwrap();
  }
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let handle = thread::spawn(move || loop {
      let job = receiver.lock().unwrap().recv().unwrap();

      println!("Worker {id} got its job, and is now executing it.");

      job();
    });

    Worker { id, handle }
  }
}
