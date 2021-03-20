mod stun_handler;
mod tcp_handler;
mod udp_handler;

fn main() {
    let t1 = udp_handler::init(String::from("127.0.0.1"));
    let t2 = tcp_handler::init(String::from("127.0.0.1"));
    t1.join().unwrap();
    t2.join().unwrap();
}
