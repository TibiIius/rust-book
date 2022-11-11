struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  //`let Some(x)` -> refutable pattern
  // only possible with `if let` and `while let`
  while let Some(e) = stack.pop() {
    println!("{e}");
  }

  let my_vec = vec!["one", "two", "three"];

  for (index, val) in my_vec.iter().enumerate() {
    println!("{index}: {val}");
  }

  let x = Some(5);

  print!("x is ");
  match x {
    Some(5) => println!("five"),
    Some(4 | 6) => println!("four or six"),
    Some(10..=100000) => println!("a big number (between 10 and 100000)"),
    // Some(y) => println!("matched something else: {y}"), // Same as below, but the matched variable is not dropped and instead is bound to y
    Some(_) => println!("something else"),
    None => println!("nothing"),
  }

  let point = Point { x: 0, y: 7 };

  // destruct so that we can use the variables x and y directly
  let Point { x, y } = point;
  assert_eq!(0, x);
  assert_eq!(7, y);

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
    (first, .., last) => {
      println!("Some numbers: {first}, {last}");
    }
  }
}
