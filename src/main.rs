mod stun_handler;
mod tcp_handler;
mod udp_handler;

/// Initiates server
/// Starts listening on UDP and TCP port 3478
fn main() {
    let t1 = udp_handler::init(String::from("0.0.0.0"));
    let t2 = tcp_handler::init(String::from("0.0.0.0"));
    t1.join().unwrap();
    t2.join().unwrap();
}
