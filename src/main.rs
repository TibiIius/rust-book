#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red,
  Blue
}

struct Inventory {
  shirts: Vec<ShirtColor>
}

impl Inventory {
  fn giveaway(&self, color: Option<ShirtColor>) -> ShirtColor {
    color.unwrap_or_else(|| self.get_most_stocked())
  }

  fn get_most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1
      }
    }

    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

fn main() {
  let store = Inventory {
    shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
  }; // 3 red, 2 blue

  let pref1 = Some(ShirtColor::Blue);
  let giveaway1 = store.giveaway(pref1);

  println!("The user with preference {:?} gets {:?}", pref1, giveaway1);

  let pref2 = None;
  let giveaway2 = store.giveaway(pref2);
  println!("The user with preference {:?} gets {:?}", pref2, giveaway2); 
}
