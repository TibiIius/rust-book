use std::fmt::Display;

fn main() {}

// tell all parameters to life as long as the lifetime of `'a` is
// otherwise, the compiler assigns lifetime `'a` => `x` && `'b` => `y` and can't determine the
// lifetime of `-> &str`
// see https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// traits, lifetimes, generics in one
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
  T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
