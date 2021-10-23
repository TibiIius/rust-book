fn main() {
  let my_coin: Coin = Coin::Nickel;
  let my_special_coin = Coin::Special(30);

  println!(
    "My coin is worth {}, my special coin is worth {} cents.",
    Coin::value_coin_cents(&my_coin),
    Coin::value_coin_cents(&my_special_coin)
  );

  let maybe_int = square_val(&Some(5));
  let not_int = square_val(&None);

  let maybe_int_il: Option<i32> = square_val_iflet(&Some(5));
  let not_int_il: Option<i32> = square_val_iflet(&None);

  std::process::exit(0);
}

fn square_val(&x: &Option<i32>) -> Option<i32> {
  match x {
    Some(v) => Some(v * v),
    _ => None,
  }
}

fn square_val_iflet(&x: &Option<i32>) -> Option<i32> {
  if let (Some(v)) = x {
    Some(v * v)
  } else {
    None
  }
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Special(u8),
}

impl Coin {
  fn value_coin_cents(coin: &Coin) -> u8 {
    match coin {
      Coin::Penny => 2,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Special(s) => *s,
    }
  }
}
