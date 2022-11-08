use std::ops::Deref;
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> MyBox<T> {
  fn new(init_val: T) -> MyBox<T> {
    MyBox(init_val)
  }
}

fn hello(name: &str) {
  println!("Hello {name}! :)");
}

fn main() {
  let my_val = MyBox::new(String::from("Tim"));
  hello(&my_val);
}
