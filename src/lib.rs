pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  // trait object; `Box<T>` can hold any type `T` that implements the `Draw` trait
  // this could hold instances of type e.g. `Box<Button>` and `Box<TextBox>` using a single `Screen` instance; using `Vec<T>` would limit us to one type per instance of `Screen` only
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for comp in self.components.iter() {
      comp.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    // do some drawing stuff
  }
}
