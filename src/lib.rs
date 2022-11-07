//! # minigrep
//!
//! `minigrep` is a CLI tool that imitates the functionality of the `grep` command.

use std::error::Error;
use std::fs;

pub struct Arguments {
  query_string: String,
  file_name: String,
}

impl Arguments {
  pub fn new(mut args: impl Iterator<Item = String>) -> Result<Arguments, &'static str> {
    args.next(); // Skip the program name

    let query_string = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let file_name = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

    Ok(Arguments {
      query_string,
      file_name,
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

/// Takes a string and a query param
///
/// Returns a vector of strings that contain the query param
///
/// # Example
/// ```
/// let query = "My";
///
/// let contents = "My house is big\nmy house is big";
///
/// assert_eq!(vec!["My house is big"], minigrep::find_lines(query, contents));
pub fn find_lines<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  content.lines().filter(|l| l.contains(query)).collect()
}

/// Takes a string and a query param
///
/// Returns a vector of strings that contain the query param, wihtout case sensitivity
///
/// # Example
/// ```
/// let query = "My";
///
/// let contents = "My house is big\nmy house is big";
///
/// assert_eq!(vec!["My house is big", "my house is big"], minigrep::find_lines_case_insensitive(query, contents));
pub fn find_lines_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  content
    .lines()
    .filter(|l| l.to_lowercase().contains(&query.to_lowercase()))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_arguments() {
    let args = vec!["program_name".to_string(), "test".to_string(), "file.md".to_string()];

    let args_parsed = Arguments::new(args.into_iter());

    assert_eq!(args_parsed.as_ref().unwrap().query_string, "test".to_string());
    assert_eq!(args_parsed.as_ref().unwrap().file_name, "file.md".to_string());
  }

  #[test]
  fn new_arguments_incorrect() {
    let args = vec!["program_name".to_string()];

    let args_parsed = Arguments::new(args.into_iter());

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
