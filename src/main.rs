use std::{thread, time::Duration};

fn main() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("Number {i} in spawned thread.");
      thread::sleep(Duration::from_millis(1));
    }
  });

  let handle2 = thread::spawn(move || {
    println!("My vector v has the beautiful values: {:?}", v);
  });

  handle2.join().unwrap();

  for i in 1..5 {
    println!("Number {i} in main thread.");
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();
}
