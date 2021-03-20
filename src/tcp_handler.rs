use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::stun_handler::process_request;

const PORT: &str = "3478";
const MAX_LENGTH: usize = 576;

pub fn init(ip: String) {
    let address = ip + ":" + PORT;
    let socket = TcpListener::bind(address).unwrap();
    listen_forever(socket);
}

fn listen_forever(socket: TcpListener) {
    for streams in socket.incoming() {
        match streams {
            Err(e) => {
                eprintln!("error: {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handler(stream);
                });
            }
        }
    }
}

fn handler(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let mut buffer = [0; MAX_LENGTH];
    loop {
        let nbytes = stream.read(&mut buffer).unwrap();

        // Write buffer to socket
        let buffer = &buffer[..nbytes];
        let buffer = process_request(buffer, stream.local_addr().unwrap());
        match stream.write(&buffer) {
            Ok(_) => {println!("Sending msg {:x?}", buffer);}
            Err(_) => {eprintln!("Error when sending tcp message {:x?}", buffer)}
        };
        stream.flush().unwrap();
    }
}
