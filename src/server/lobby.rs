use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use std::str;
use std::time::Duration;
use std::io::{ErrorKind};

struct Client {
    id : i32,
    stream : TcpStream,
    pseudo : String
}

fn handle_messages(clients_sharable : Arc<Mutex<Vec<Client>>>) {

    let mut client_message = String::new(); 
    let mut remove_ids = Vec::new();

    loop {
        {
            let mut clients = clients_sharable.lock().unwrap();

            if clients.len() != 0 {
                println!("{} clients connected !", clients.len());
            }

            for client in clients.iter_mut() {
                let peer_addr = match client.stream.peer_addr() {
                    Err(e) => {println!("{}", e); let server : SocketAddr = "0.0.0.0:0".parse().unwrap(); server},
                    Ok(addr) => {addr}

                };
                println!("Client : {} connected with id {}", peer_addr, client.id);
                
                let buffer : &mut[u8] = &mut[0;100];
                match client.stream.read(buffer) {
                    // Error if nothing to read is normal because of non-blocking read
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        println!("Nothing to read from {}", client.pseudo);
                    },
                    Err(e) => {
                        println!("Error from client: {}", e);
                        println!("Error from client: {:?}", e.kind());
                        println!("Client disconnected");
                        remove_ids.push(client.id);
                    },
                    Ok(nb) => {

                        if nb != 0 {
                            client_message.push_str(str::from_utf8(buffer).unwrap());
                            println!("Client : {} said : {}", client.pseudo, client_message);
                            let _ = client.stream.write("Ack".as_bytes());
                        }
                        // Client disconnected, remove it
                        else {
                            println!("Client disconnected");
                            remove_ids.push(client.id);
                        }
                    }
                }
            }

            // Remove disconnected clients 
            for remove_id in remove_ids.iter(){
                clients.retain(|ref x| x.id != *remove_id);
                println!("Client with id : {} removed", remove_id);
            }
            remove_ids.clear();
            client_message.clear();
        }
        thread::sleep_ms(200);
    }

}

pub fn lobby() {

    let listener = TcpListener::bind("127.0.0.1:50050").unwrap();
    let clients_sharable = Arc::new(Mutex::new(Vec::<Client>::new()));

    let clone = clients_sharable.clone();
    println!("Message handler spawning");
    thread::spawn(move || handle_messages(clone));
    let mut ids = 0;

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
                let client = Client {id: ids, stream : stream, pseudo : pseudo.clone()};
                let _ = client.stream.set_read_timeout(Some(Duration::from_millis(10)));

                clients.push(client);
                ids += 1;
            }
            Err(e) => {
                println!("Error in lobby listener : {}", e);
            }
        }
    }

    drop(listener);
}

