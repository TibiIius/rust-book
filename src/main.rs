fn main() {
  let s = String::from("kekw");

  println!("first word of s: {}", first_word(&s[..]));
  // Does the same thing
  // println!("first word of s: {}", first_word(&s));
}

fn first_word(s: &str) -> &str {
  println!("inside 'first_word' - s is: {}", s);

  let s_bytes = s.as_bytes();

  for (i, &v) in s_bytes.iter().enumerate() {
    if v == b' ' {
      return &s[0..i];
    }
  }

  return &s;
}
