use rand::{random, seq::SliceRandom};
use std::{io::Write, net::TcpStream, thread, time::Duration};

const MESSAGE_DIC: [&str; 5] = [
    "Rustが好きです",
    "Rustはコンパイラが厳しいです",
    "Rustはコンパイラが速いです",
    "Rustはコンパイラが親切です",
    "カレー食べたい",
];

fn main() {
    let host = env::get_ip_address();
    let port = env::get_default_port();

    let Ok(mut stream) = TcpStream::connect(format!("{}:{}", host, port)) else {
        println!(
            "Trying to connect to the default server at {}:{}",
            host, port,
        );
        panic!("Could not connect to the server or the server is not running");
    };

    println!("Connected to the server at {}:{}", host, port);

    loop {
        let id: u8 = random();

        let selected = MESSAGE_DIC.choose(&mut rand::thread_rng()).unwrap();
        let selected = format!("[{}] {}\n", id, selected);

        print!("Send: {}", selected);

        if stream.write(selected.as_bytes()).is_err() {
            break;
        };

        stream.flush().ok();
        thread::sleep(Duration::from_secs(3));
    }

    println!("Connection closed.");
}
