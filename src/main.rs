use std::{
  sync::{Arc, Mutex},
  thread,
};

fn main() {
  // Arc is basically an atomic Rc, so it's thread-safe
  let rc_mtx = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let mtx = Arc::clone(&rc_mtx);
    let handle = thread::spawn(move || {
      let mut guard = mtx.lock().unwrap();

      *guard += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("Result: {}", *rc_mtx.lock().unwrap());
}
