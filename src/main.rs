use std::collections::HashMap;

fn main() {
  let teams = vec!["Yellow", "Blue"];
  let mut scores = vec![1337, 1336];

  let mut my_hash_map: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

  my_hash_map.entry("Red").or_insert(1);

  println!("{:?}", my_hash_map);

  match my_hash_map.get("Yellow") {
    Some(e) => println!("Yellow team's points are: {}", e),
    None => println!("No yellow team! :("),
  }
}
