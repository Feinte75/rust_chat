use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::io;
use std::io::{ErrorKind};
use std::time::Duration;
use std::thread;
use std::sync::mpsc::{channel, Sender, TryRecvError};

fn get_user_input(tx : Sender<String>) {

    let mut buffer = String::new();
    loop {

        let _ = io::stdin().read_line(&mut buffer);
        let _ = tx.send(buffer.clone());
        buffer.clear();
        thread::sleep(Duration::from_millis(50));
    }
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

    let (tx, rx) = channel();

    thread::spawn(move || 
        
        get_user_input(tx.clone())
    );

    let mut input_message = String::new();
    loop {

        match rx.try_recv() {
            Err(e) if e == TryRecvError::Empty => {
                continue;
            },

            Err(e) => {
                println!("{}", e);
                break;
            }
            Ok(input) => {
                input_message.push_str(str::from_utf8(input.as_bytes()).unwrap());        
            }
        }

        let _ = stream.write(&mut input_message.as_bytes());

        match stream.read(&mut buf) {
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(e) =>  {
                println!("{}", e);
                break;
            },
            Ok(_) => ()
        }

        println!("Server response : {}", str::from_utf8(&buf).unwrap());
        input_message.clear();
        thread::sleep(Duration::from_millis(100));
    }
}
