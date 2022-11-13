struct Human;

struct Dog;

trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

trait Animal {
  fn baby_name() -> String;
}

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("We Morrowind now.");
  }
}

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

fn main() {
  let person = Human;
  person.fly();
  Pilot::fly(&person);
  Wizard::fly(&person);

  println!("A name for a baby dog could be \"{}.\"", Dog::baby_name());
  println!("A baby dog is called a {}.", <Dog as Animal>::baby_name());
}
