use std::{
  sync::{mpsc, Arc, Mutex},
  thread,
};

struct Worker {
  id: usize,
  handle: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  threads: Vec<Worker>,
  sender: Option<mpsc::Sender<Job>>,
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

    ThreadPool {
      threads,
      sender: Some(sender),
    }
  }

  pub fn execute<F>(&self, func: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(func);

    self.sender.as_ref().unwrap().send(job).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    drop(self.sender.take());

    for thread in &mut self.threads {
      println!("Worker with ID '{}' is shutting down", thread.id);

      if let Some(h) = thread.handle.take() {
        h.join().unwrap();
      }
    }
  }
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let handle = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv();

      match message {
        Ok(job) => {
          println!("Worker {id} got its job, and is now executing it.");

          job();
        }
        Err(_) => {
          println!("Worker {id} disconnected and shutting down.");
          break;
        }
      }
    });

    Worker {
      id,
      handle: Some(handle),
    }
  }
}
