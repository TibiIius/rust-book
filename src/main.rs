fn main() {
  let user_1 = User {
    username: "Tim1337".to_string(),
    password: "s3cr3t!".to_string(),
    login_count: 0,
  };

  let user_2 = User {
    username: "kekw_h4ckz0r".to_string(),
    password: "123".to_string(),
    ..user_1
  };

  println!(
    "{}:\npassword: {}\nlogin count: {}",
    user_1.username, user_1.password, user_1.login_count
  );

  println!(
    "{}:\npassword: {}\nlogin count: {}",
    user_2.username, user_2.password, user_2.login_count
  );
}

struct User {
  username: String,
  password: String,
  login_count: u64,
}
