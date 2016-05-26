extern crate chat;

use std::env;
use chat::client::client;
use chat::server::lobby;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            if args[1] == "server" {
                lobby::lobby();
            }
            else if args[1] == "client" {
                client::client();
            }
        }
        _ => {
            println!("invalid parameter");
        }
    }
}

