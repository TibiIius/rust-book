fn main() {
  let my_coin: Coin = Coin::Nickel;

  let my_special_coin = Coin::Special(30);

  println!(
    "My coin is worth {}, my special coin is worth {} cents.",
    Coin::value_coin_cents(&my_coin),
    Coin::value_coin_cents(&my_special_coin)
  );
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
