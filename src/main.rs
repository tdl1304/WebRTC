use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let t1 = thread::spawn(move || {
        while (true) {
            sleep(Duration::from_millis(5000));
            println!("Trying to connect");
            let stream = TcpStream::connect("127.0.0.1:8080");
        }
    });

    println!("b4");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("aft");
    // accept connections and process them serially
    while(true) {
        let client = listener.accept().unwrap();
        println!("Client connected {}", client.1.to_string())
    }


}



