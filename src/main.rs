fn main() {
  let s1 = "me a string".to_string();
  let s2 = ", me too!".to_string();

  let s_combined = s1 + &s2;

  let s1 = "me a string".to_string();

  let s_formatted = format!("{}{}", s1, s2);

  println!("{}", s_formatted);

  print!("{}", s1);
  print!("{}", s2);
}
