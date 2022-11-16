use rust_web_server::ThreadPool;
use std::{
  fs,
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
  thread,
  time::Duration,
};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").expect("Binding failed, check for port availability");
  let pool = ThreadPool::new(8);

  for stream in listener.incoming().take(2) {
    let stream = stream.unwrap();

    pool.execute(|| {
      parse_conn(stream);
    });
  }

  println!("Shutting down main thread");
}

fn parse_conn(mut stream: TcpStream) {
  let reader = BufReader::new(&mut stream);

  let req = reader.lines().next().unwrap().unwrap();

  let (status, filename) = match &req[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/index.html"),
    "GET /sleep HTTP/1.1" => {
      thread::sleep(Duration::from_secs(5));
      ("HTTP/1.1 200 OK", "src/index.html")
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html"),
  };

  let body = fs::read_to_string(filename).unwrap();
  let len = body.len();

  let res = format!("{status}\r\nContent-Length: {len}\r\n\r\n{body}");

  stream.write_all(res.as_bytes()).unwrap()
}
