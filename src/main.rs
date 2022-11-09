use std::{process, sync::mpsc, thread, time::Duration};

fn main() {
  let (tx, rx) = mpsc::channel();
  let tx2 = tx.clone();

  thread::spawn(move || {
    let vals = vec![String::from("Hello"), String::from("from"), String::from("sender.")];

    for val in vals {
      if let Err(e) = tx.send(val) {
        println!("An error occured trying to send some data to a receiver: {e}");
      }
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![String::from("How"), String::from("are"), String::from("you?")];

    for val in vals {
      if let Err(e) = tx2.send(val) {
        println!("An error occured trying to send some data to a receiver: {e}");
      }
      thread::sleep(Duration::from_secs(1));
    }
  });

  // `recv()` blocks until the transmitter sent something
  // `try_recv()` would not block and return Err if it got nothing from the transmitter
  // let received = rx.recv().unwrap();

  // basically calls recv() a couple of times
  for received in rx {
    println!("Received some data from another thread: {received}");
  }

  process::exit(0)
}
