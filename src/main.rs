mod stun_handler;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let something = stun_handler::Headers {
        message_type: 0,
        message_length: 0,
        magic_cookie: 0,
        transaction_id: 0,
    };

    let t1 = thread::spawn(move || {
        loop {
            sleep(Duration::from_millis(5000));
            println!("Trying to connect");
            let stream = TcpStream::connect("127.0.0.1:8080");
        }
    });

    println!("b4");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("aft");
    // accept connections and process them serially
    loop {
        let client = listener.accept().unwrap();
        println!("Client connected {}", client.1.to_string())
    }


}



