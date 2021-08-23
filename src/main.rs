fn main() {
  let s = String::from("kekw");

  // takes_ownership(s);
  // let s = takes_and_gives_ownership(s);
  // let (s, s_size) = ownership_size(s);
  let s_size = reference_size(&s);

  println!("String \"{}\" has size {}", s, s_size);
}

fn takes_ownership(s: String) {
  //
}

fn takes_and_gives_ownership(s: String) -> String {
  return s;
}

fn ownership_size(s: String) -> (String, usize) {
  let len = s.len();
  return (s, len);
}

fn reference_size(s: &String) -> usize {
  let len = s.len();
  return len;
}
