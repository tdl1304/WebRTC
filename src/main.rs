mod stun_handler;
mod tcp_handler;
mod udp_handler;

fn main() {
    udp_handler::init(String::from("127.0.0.1"));
    tcp_handler::init(String::from("127.0.0.1"));
}
