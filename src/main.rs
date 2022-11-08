use crate::List::{Cons, Nil};
use std::rc::Rc;

// this won't work as a would be moved into b and c, and can't have multiple owners
// enum List {
//   Cons(i32, Box<List>),
//   Nil,
// }

// fn main() {
// let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
// let b = Cons(3, Box::new(a));
// let c = Cons(4, Box::new(a));
// }

enum List {
  Cons(i32, Rc<List>),
  Nil,
}

fn main() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  // Rc::clone() is not a deep copy, it only increases the ref count
  let b = Cons(3, Rc::clone(&a));
  println!("count after creating a = {}", Rc::strong_count(&a));
  {
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
