use std::error::Error;
use std::fs;

pub struct Arguments {
  query_string: String,
  file_name: String,
}

impl Arguments {
  pub fn new(args: &[String]) -> Result<Arguments, &str> {
    if args.len() <= 2 {
      return Err("Not enough arguments provided!");
    }

    Ok(Arguments {
      query_string: args.get(1).unwrap().clone(),
      file_name: args.get(2).unwrap().clone(),
    })
  }
}

pub fn run(parsed_args: Arguments) -> Result<(), Box<dyn Error>> {
  println!(
    "Searching for \"{}\" in file \"{}\"",
    parsed_args.query_string, parsed_args.file_name
  );

  let contents = fs::read_to_string(parsed_args.file_name)?;

  println!("The file consists of the following:\n{}", contents);

  Ok(())
}
