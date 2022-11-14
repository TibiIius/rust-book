// create aliases for verbose types
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}

fn returns_long_type() -> Thunk {
  Box::new(|| println!("hi"))
}
fn main() {
  let f: Thunk = Box::new(|| println!("hi"));
}
