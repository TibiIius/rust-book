use std::{
  process,
  sync::{mpsc, Mutex},
  thread,
  time::Duration,
};

fn main() {
  let mtx = Mutex::new(5);

  {
    // `lock()` tries to acquire the mutex' lock
    // if the mutex is locked, it blocks until it can acquire the lock
    let mut guard = mtx.lock().unwrap();
    *guard = 6;
  }

  println!("mtx = {:?}", mtx);
}
