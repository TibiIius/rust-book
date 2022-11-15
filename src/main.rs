// make macro available to crates bringing this crate into scope
#[macro_export]
macro_rules! vec {
  // `$` -> declares a macro variable
  // `expr` matches any Rust expression, and `$x:expr` assings the Rust expression to the macro variable `$x`
  // the `,` is just literally a comma in the expression
  // the `*` indicates to match the preceding pattern 0 or more times
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      // `$()*` generates code for each repetition of the pattern
      $(
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}

fn main() {
  let t = vec![1, 2, 3];
}
