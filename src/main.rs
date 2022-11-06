fn main() {
  let mut list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let mut borrows_mutably = || list.push(7);

  // Cannot be called as the closure borrows the list mutably!
  // println!("Before calling closure: {:?}", list);

  borrows_mutably();
  println!("After calling closure: {:?}", list);
}
