fn main() {
  let mut user_vec: Vec<User> = Vec::new();

  let user_1 = User {
    username: "Tim1337".to_string(),
    password: "s3cr3t!".to_string(),
    login_count: 0,
  };

  // One line only
  println!("[DEBUG] user_1 is: {:?}", user_1);
  // Span over multiple lines
  println!("[DEBUG] user_1 is: {:#?}", user_1);

  let user_2 = User {
    username: "kekw_h4ckz0r".to_string(),
    password: "123".to_string(),
    ..user_1
  };

  user_vec.push(user_1);
  user_vec.push(user_2);

  for u in user_vec.iter() {
    print_user(u);
  }
}

fn print_user(user: &User) {
  println!("==============================");
  println!(
    "Printing out information for user {}:\npassword: {}\nlogin count: {}",
    user.username, user.password, user.login_count
  );
  println!("==============================");
}

#[derive(Debug)]
struct User {
  username: String,
  password: String,
  login_count: u64,
}
