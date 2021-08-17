use rand::Rng;

fn main() {
  println!("Guess a number game!");

  let secret_number = rand::thread_rng().gen_range(1..=100);

  loop {
    let mut user_input = String::new();

    println!("Enter a number:");

    std::io::stdin().read_line(&mut user_input).expect("KEKWait");

    let user_input: i32 = match user_input.trim().parse() {
      Ok(n) => n,
      Err(_) => {
        println!("Couldn't parse number SadGe");
        continue;
      }
    };

    println!("You entered {}!", &user_input);

    match user_input.cmp(&secret_number) {
      std::cmp::Ordering::Equal => {
        println!("Sick bro");
        break;
      }
      std::cmp::Ordering::Greater => println!("You chomk"),
      std::cmp::Ordering::Less => println!("No gainz"),
    }
  }
}
