use std::{
  fs,
  io::{prelude::*, BufReader},
  net::{TcpListener, TcpStream},
};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").expect("Binding failed, check for port availability");

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    parse_conn(stream);
  }
}

fn parse_conn(mut stream: TcpStream) {
  let reader = BufReader::new(&mut stream);

  let req = reader.lines().next().unwrap().unwrap();

  let (status, filename) = if req == "GET / HTTP/1.1" {
    ("HTTP/1.1 200 OK", "src/index.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "src/404.html")
  };

  let body = fs::read_to_string(filename).unwrap();
  let len = body.len();

  let res = format!("{status}\r\nContent-Length: {len}\r\n\r\n{body}");

  stream.write_all(res.as_bytes()).unwrap()
}
