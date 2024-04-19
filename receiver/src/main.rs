use std::{
    io::BufRead,
    net::{TcpListener, TcpStream},
};

fn main() {
    let host = env::get_ip_address();
    let port = env::get_default_port();

    let Ok(listner) = TcpListener::bind(format!("{}:{}", host, port)) else {
        panic!("Could not connect to server or bind to port.");
    };

    println!("Server is listening on {}:{}", host, port);

    for stream in listner.incoming() {
        let Ok(stream) = stream else {
            break;
        };
        handle(stream);
    }

    println!("Connection closed.")
}

fn handle(stream: TcpStream) {
    let mut buf = String::new();
    let mut buf_read = std::io::BufReader::new(stream);

    while buf_read.read_line(&mut buf).is_ok() {
        print!("Recieve: {}", buf);
        buf.clear();
    }
}
