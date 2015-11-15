use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) {
    let _ = stream.write(b"ok !");
}

pub fn lobby() {

    let listener = TcpListener::bind("127.0.0.1:50050").unwrap();

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    println!("Incoming connection");
                    handle_client(stream);
                });
            }
            Err(e)
                => {
                    println!("Error in lobby listener");
                }
        }
    }

    drop(listener);
}

fn main() {
    lobby();
}
