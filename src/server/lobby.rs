use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::str;
use std::time::Duration;
use std::io::{ErrorKind};

struct Client {
    stream : TcpStream,
    pseudo : String
}

fn handle_messages(clients_sharable : Arc<Mutex<Vec<Client>>>) {

    let mut client_message = [0 ; 100];
    loop {
        {
            let mut clients = clients_sharable.lock().unwrap();
            if clients.len() != 0 
            {
                //                println!("Mutex acquired, {} clients connected !", clients.len());
                for client in clients.iter_mut() {
                    let peer_addr = client.stream.peer_addr().unwrap();
                    //                    println!("{}", peer_addr);
                    let _ = client.stream.set_read_timeout(Some(Duration::from_millis(10)));

                    match client.stream.read(&mut client_message) {
                        Err(e) => {
                            if e.kind() == ErrorKind::ConnectionAborted {

                                println!("SHIIIIT3");
                                println!("oups: {}", e);
                            } else {
                                println!("Nope");
                            }
                        },
                        Ok(nb) => {

                            if nb != 0 {
                                println!("{}", str::from_utf8(&client_message).unwrap());
                                let _ = client.stream.write(client.pseudo.as_bytes());
                            }
                        }
                    }
                }
            }
        }
        thread::sleep_ms(50);
    }

}

pub fn lobby() {

    let listener = TcpListener::bind("127.0.0.1:50050").unwrap();
    let clients_sharable = Arc::new(Mutex::new(Vec::<Client>::new()));

    let clone = clients_sharable.clone();
    println!("Message handler spawning");
    thread::spawn(move || handle_messages(clone));

    println!("Listening ...");
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut pseudo = String::new();
                println!("Reading !");
                let mut b = [0 ; 8];
                let _ = stream.read(&mut b);
                pseudo.push_str(str::from_utf8(&b).unwrap());
                println!("Recieved pseudo : {}", pseudo);
                let _ = stream.write(&mut pseudo.as_bytes());

                let mut clients = clients_sharable.lock().unwrap();
                println!("Pushing new client in Vec !");
                clients.push(Client {stream : stream, pseudo : pseudo.clone()});
            }
            Err(e)
                => {
                    println!("Error in lobby listener");
                }
        }
    }

    drop(listener);
}

