fn main() {
  let x = 5;

  let y: Option<i32> = Some(5);

  println!("{}", x + y.unwrap());
}
