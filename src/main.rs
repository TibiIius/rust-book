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

  let req: Vec<_> = reader
    .lines()
    .map(|res| res.unwrap())
    .take_while(|l| !l.is_empty())
    .collect();

  println!("Request object: {:#?}", req);

  let status = "HTTP/1.1 200 OK";
  let body = fs::read_to_string("src/index.html").unwrap();
  let len = body.len();

  let res = format!("{status}\r\nContent-Length: {len}\r\n\r\n{body}");

  stream.write_all(res.as_bytes()).unwrap()
}
