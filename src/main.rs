fn main() {
  let mut s = String::from("kekw");

  let r1 = &mut s;
  r1.push_str(" < lulw");
  println!("{}", r1);

  let r2 = &mut s;
  r2.push_str(" pepeLaugh");
  println!("{}", r2);
}
