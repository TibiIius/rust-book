use std::fs::File;
use std::io::ErrorKind;

fn main() {
  let f = File::open("./test.md");

  let f = match f {
    Ok(f) => f,
    Err(e) => match e.kind() {
      ErrorKind::NotFound => {
        println!("File does not yet exist and will be created");
        match File::create("./test.md") {
          Ok(fc) => fc,
          Err(e) => panic!("Error creating file: {:?}", e),
        }
      }
      other => panic!("Error reading/creating file: {:?}", other),
    },
  };
}
