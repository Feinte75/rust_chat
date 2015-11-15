use std::io::prelude::*;
use std::net::TcpStream;

pub fn client() {
    let mut buffer = String::new();

    let mut stream = TcpStream::connect("127.0.0.1:50050").unwrap();
    let _ = stream.write(b"test");
    let _ = stream.read_to_string(&mut buffer);

    println!("{}", buffer);
}
