use env;
use std::{io::Write, net::TcpStream, thread, time::Duration};

fn main() {
    let host = env::get_ip_address();
    let port = env::get_default_port();

    let Ok(mut stream) = TcpStream::connect(format!("{}:{}", host, port)) else {
        panic!("Could not connect to the server or the server is not running");
    };

    while let Ok(_) = stream.write(b"Hello, World!") {
        thread::sleep(Duration::from_secs(1));
    }
}
