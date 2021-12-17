use std::{thread, time::Duration};

fn main() {
  let simulated_user_specified_value = 10;
  let simulated_random_number = 7;

  generate_workout_plan(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: i32) -> i32 {
  println!("Doing slow calculations...");
  thread::sleep(Duration::from_secs(3));
  println!("Done calculating!");
  intensity
}

fn generate_workout_plan(intensity: i32, random_number: i32) {
  if intensity < 25 {
    println!("Today, do {} pushups!", simulated_expensive_calculation(intensity));
    println!("Next, do {} situps!", simulated_expensive_calculation(intensity));
  } else if random_number == 3 {
    println!("Take a break today! Remember to stay hydrated!");
  } else {
    println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
  }
}
