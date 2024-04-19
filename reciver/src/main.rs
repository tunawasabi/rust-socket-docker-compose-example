use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use env;

fn main() {
    let host = env::get_ip_address();
    let port = env::get_default_port();

    let Ok(listner) = TcpListener::bind(format!("{}:{}", host, port)) else {
        panic!("Could not connect to server or bind to port.");
    };

    for stream in listner.incoming() {
        let Ok(stream) = stream else {
            break;
        };
        handle(stream);
    }

    println!("Connection closed.")
}

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).ok();

    println!("Recieve: {}", String::from_utf8_lossy(&buffer[..]));
}
