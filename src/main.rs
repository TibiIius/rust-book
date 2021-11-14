use minigrep::Arguments;

fn main() {
  let parsed_args = Arguments::new(std::env::args().collect::<Vec<String>>().as_ref()).unwrap_or_else(|e| {
    println!("Error parsing console arguments: {}", e);
    std::process::exit(1);
  });

  // Use if let instead of unwrap_or_else() because we don't want to unwrap Ok(()), we only care about the error value
  if let Err(e) = minigrep::run(parsed_args) {
    println!("The program encountered an unexpected error: {}", e);

    std::process::exit(1);
  }

  std::process::exit(0);
}
