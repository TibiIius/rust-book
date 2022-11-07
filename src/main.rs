fn main() {
  let list: Vec<i32> = vec![1, 2, 3];

  println!("Original list:");
  for val in list.iter() {
    print!("{} ", val);
  }
  println!();

  let list2: Vec<_> = list.iter().map(|x| x + 1).collect();

  println!("Modified list:");
  for val in list2.iter() {
    print!("{} ", val);
  }
  println!();
}
