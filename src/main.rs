// Can't compile like this
// Reason is that Rust tries to find the largest possible value in the enum, and since `Cons(i32, List)` can be infinitely big, the size can't be determined
// enum List {
//   Cons(i32, List),
//   Nil,
// }

enum List {
  Cons(i32, Box<List>),
  Nil,
}

use crate::List::{Cons, Nil};

fn main() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

  let x = 5;
  let y = Box::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);
}
