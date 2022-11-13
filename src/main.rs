// static variables are basically like constants
// unlike constants though, they always refer to the same value in memory, meaning they can never be copied
// accessing static mutable variables is considered unsafe
// they are considered unsafe as mutiple parts of the program always access the same instance
// with mutiple threads, accessing static variables and mutating them can lead to data races
static mut COUNTER: u32 = 0;

fn add_to(inc: u32) {
  unsafe {
    COUNTER += inc;
  }
}
fn main() {
  add_to(3);

  unsafe {
    println!("COUNTER has the value: {COUNTER}");
  }
}
