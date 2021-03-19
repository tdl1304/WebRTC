use crate::stun_handler::*;
use std::net::UdpSocket;

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
        let (validation, transaction_id) = process_request(buffer);
        if validation {
            success_response(src_address, transaction_id);
        } else {

        }
        //buffer.reverse();
        //socket.send_to(buffer, &src_address);
    }
}


