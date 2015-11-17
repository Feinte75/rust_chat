use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::str;

struct Client {
    stream : TcpStream,
    pseudo : String
}

fn handle_messages(mut clients_sharable : Arc<Mutex<Vec<Client>>>) {
    let mut nb_clients = 0;
    loop {

        {
            let mut clients = clients_sharable.lock().unwrap();
            if clients.len() != 0 
            {
                if nb_clients != clients.len() {
                    nb_clients = clients.len();
                    println!("Mutex acquired, {} clients connected !", clients.len());
                    for client in clients.iter_mut() {

                        //let mut client = clients.pop().unwrap();
                        let _ = client.stream.write(client.pseudo.as_bytes());
                    }
                }
            }
        }
        thread::sleep_ms(50);
    }
}

pub fn lobby() {

    let listener = TcpListener::bind("127.0.0.1:50050").unwrap();
    let mut clients_sharable = Arc::new(Mutex::new(Vec::<Client>::new()));

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

