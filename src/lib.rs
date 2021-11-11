#[cfg(test)]
mod tests {
  use crate::{i_will_kill, print_stuff};

  #[test]
  fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err("reingekekt".to_string())
    }
  }

  #[test]
  fn explode() {
    let res = print_stuff();

    assert!(
      res.contains("Hello"),
      "Test failed, the function's return value was: {}",
      res
    )
  }

  #[test]
  #[should_panic]
  fn explode_2() {
    i_will_kill();
  }
}

fn print_stuff() -> String {
  "Hallo".to_string()
}

fn i_will_kill() {
  panic!("haha");
}
