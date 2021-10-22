fn main() {
  let my_coin: Coin = Coin::Nickel;

  println!("{}", Coin::value_coin_cents(&my_coin));
}

enum Coin {
  Penny,
  Nickel,
  Dime,
}

impl Coin {
  fn value_coin_cents(coin: &Coin) -> u8 {
    match coin {
      Coin::Penny => 2,
      Coin::Nickel => 5,
      Coin::Dime => 10,
    }
  }
}
