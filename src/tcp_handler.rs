use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::stun_handler::process_request;
use std::thread::JoinHandle;
use std::time::Duration;

const PORT: &str = "3478";
const MAX_LENGTH: usize = 576;

/// Initiate server to listen for TCP on port 3478
///
/// Creates a new thread for every connection, every connection has a timeout of 5000 ms
///
/// Returns joinhandle of the listener
pub fn init(ip: String) -> JoinHandle<()> {
    let address = ip + ":" + PORT;
    let socket = TcpListener::bind(address).unwrap();
    listen_forever(socket)
}

fn listen_forever(socket: TcpListener) -> JoinHandle<()> {
    thread::spawn(move || {
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
    })
}

fn handler(mut stream: TcpStream) {
    stream
        .set_read_timeout(Option::from(Duration::from_millis(5000)))
        .unwrap();
    println!("Connection from {}", stream.peer_addr().unwrap());
    let mut buffer = [0; MAX_LENGTH];
    loop {
        let nbytes = stream.read(&mut buffer).unwrap();

        // Write buffer to socket
        let buffer = &buffer[..nbytes];
        let buffer = process_request(buffer, stream.local_addr().unwrap());
        match stream.write(&buffer) {
            Ok(_) => {
                println!(
                    "Sending msg to {}, {:x?}",
                    stream.peer_addr().unwrap(),
                    buffer
                );
            }
            Err(_) => {
                eprintln!(
                    "Error when sending udp message to {}, {:x?}",
                    stream.peer_addr().unwrap(),
                    buffer
                );
            }
        };
        stream.flush().unwrap();
    }
}
