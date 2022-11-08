struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data: `{}`.", self.data);
  }
}

fn main() {
  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };

  let c2 = CustomSmartPointer {
    data: String::from("other stuff"),
  };

  println!("CustomSmartPointer instances created.");

  std::mem::drop(c2);
  println!("c2 dropped before end of main.");
}
