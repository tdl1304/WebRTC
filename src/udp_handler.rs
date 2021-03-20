use std::net::UdpSocket;
use std::thread;
use crate::stun_handler::process_request;

const PORT: &str = "3478";
const MAX_LENGTH: usize = 576;

pub fn init(ip: String) {
    let address = ip + ":" + PORT;
    let socket: UdpSocket = UdpSocket::bind(address).unwrap();
    let mut buf = [0; MAX_LENGTH];
    thread::spawn(move || listen_forever(socket, &mut buf));
}

fn listen_forever(socket: UdpSocket, mut buffer: &mut [u8]) {
    loop {
        let (length, src_address) = socket.recv_from(&mut buffer).expect("no data received");
        let buffer = &buffer[..length];
        let buffer = process_request(buffer, src_address);
        match socket.send_to(&buffer, src_address) {
            Ok(_) => {println!("Sending msg {:x?}", buffer);}
            Err(_) => {eprintln!("Error when sending udp message {:x?}", buffer)}
        };
    }
}
