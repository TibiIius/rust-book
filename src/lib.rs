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
  let contents = fs::read_to_string(parsed_args.file_name)?;

  let res = if std::env::var("CASE_INSENSITIVE").is_err() {
    find_lines(parsed_args.query_string.as_ref(), contents.as_ref())
  } else {
    find_lines_case_insensitive(parsed_args.query_string.as_ref(), contents.as_ref())
  };

  for l in res {
    println!("{}", l);
  }

  Ok(())
}

pub fn find_lines<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut ret: Vec<&str> = vec![];

  for l in content.lines() {
    if l.contains(query) {
      ret.push(l);
    }
  }

  ret
}

pub fn find_lines_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut ret: Vec<&str> = vec![];

  for l in content.lines() {
    if l.to_lowercase().contains(&query.to_lowercase()) {
      ret.push(l)
    }
  }

  ret
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_arguments() {
    let args = vec!["program_name".to_string(), "test".to_string(), "file.md".to_string()];

    let args_parsed = Arguments::new(args.as_ref());

    assert_eq!(args_parsed.as_ref().unwrap().query_string, "test".to_string());
    assert_eq!(args_parsed.as_ref().unwrap().file_name, "file.md".to_string());
  }

  #[test]
  fn new_arguments_incorrect() {
    let args = vec!["program_name".to_string()];

    let args_parsed = Arguments::new(args.as_ref());

    assert!(args_parsed.is_err());
  }

  #[test]
  fn single_result() {
    let query = "duct";

    let file_content = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], find_lines(query, file_content));
  }
}
