fn main() {
  let number_list = vec![1, 2, 3, 4];

  let char_list = vec!['a', 'b', 'C'];

  println!("the largest number in the list is: {}", largest(&number_list));

  println!("the largest char in the list is: {}", largest(&char_list));

  let point = Point { x: 5, y: 7 };
  let point_other = Point { x: 5, y: 7 };

  println!("it is {} that both points are equal.", point.equals(&point_other));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T
where
  T: Copy,
{
  let mut largest = list[0];

  for &i in list {
    if i > largest {
      largest = i;
    }
  }

  largest
}

struct Point<T: std::cmp::PartialEq> {
  x: T,
  y: T,
}

impl<T: std::cmp::PartialEq> Point<T> {
  fn equals(&self, other: &Point<T>) -> bool {
    self.y == other.y && self.x == other.x
  }
}
