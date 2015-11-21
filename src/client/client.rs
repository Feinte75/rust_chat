use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::io;
use std::io::{ErrorKind};
use std::time::Duration;
use std::thread;

fn get_user_input() -> String {
    
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    return buffer;
}

pub fn client() {

    println!("Connecting !");
    let mut stream = TcpStream::connect("127.0.0.1:50050").unwrap();

    println!("Writing !");
    let _ = stream.write(b"John");

    println!("Reading !");
    let mut buf = [0;10];
    let _ = stream.read(&mut buf);
    let _ = stream.set_read_timeout(Some(Duration::from_millis(50))); 
    loop {
        let input_message = get_user_input();
        
        let _ = stream.write(&mut input_message.as_bytes());

        match stream.read(&mut buf) {
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(_) => break,
            Ok(_) => ()
        }

        println!("Server response : {}", str::from_utf8(&buf).unwrap());
        thread::sleep_ms(100);
    }

}
