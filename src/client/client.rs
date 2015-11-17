use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

pub fn client() {
    let mut buffer = String::new();

    println!("Connecting !");
    let mut stream = TcpStream::connect("127.0.0.1:50050").unwrap();
    println!("Writing !");
    let _ = stream.write(b"John");
    let _ = stream.flush();
    println!("Reading !");
    loop {
        let mut buf = [0;10];

        let _ = stream.read(&mut buf);
        println!("{}", str::from_utf8(&buf).unwrap());
        buffer.push_str(str::from_utf8(&buf).unwrap());
    }

//    println!("{}", buffer);
}
