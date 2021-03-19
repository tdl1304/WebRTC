use crate::stun_handler::*;
use std::net::UdpSocket;
use std::process::exit;
use std::fmt::{LowerHex, Formatter};

const PORT: &str = "3478";
const MAX_LENGTH: usize = 576;

pub fn init(ip: String) {
    let address = ip + ":" + PORT;
    let socket:UdpSocket = UdpSocket::bind(address).unwrap();
    let mut buf = [0; MAX_LENGTH];
    listen(socket,&mut buf);
}

fn listen(socket:UdpSocket, mut buffer: &mut [u8]) {
    loop {
        let (length, src_address) = socket.recv_from(&mut buffer).expect("no data received");
        let buffer = &mut buffer[..length];
        println!("{}", buffer.len());
        processUDP(buffer);
        //buffer.reverse();
        //socket.send_to(buffer, &src_address);
    }
}

fn processUDP(mut buffer: &mut[u8]) {
    if buffer.len() < 20 {
        //return error
    }
    // Create headers
    let headers = Headers {
        message_type: format_bytes16(&mut buffer[0..2]),
        message_length: format_bytes16(&mut buffer[2..4]),
        magic_cookie: format_bytes32(&mut buffer[4..8]),
        transaction_id: format_bytes96(&mut buffer[8..20]),
    };

    if validate_headers(headers) {
        //When headers are valid

    } else {
        //When headers are not valid

    }

    println!("{:?}", headers)
}
