use std::vec;

use gui::{Button, Draw, Screen};

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // some other draw stuff
  }
}

fn main() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 12,
        height: 20,
        options: vec![String::from("yes"), String::from("no"), String::from("we yes no")],
      }),
      Box::new(Button {
        width: 10,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };
}
