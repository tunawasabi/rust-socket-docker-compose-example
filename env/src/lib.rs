const DEFAULT_HOST: &str = "localhost";
const DEFAULT_PORT: u16 = 41222;

pub fn get_ip_address() -> String {
    match std::env::var("EXAMPLE_HOST") {
        Ok(ip) => ip,
        Err(_) => DEFAULT_HOST.to_string(),
    }
}

pub fn get_default_port() -> u16 {
    let port = std::env::var("EXAMPLE_PORT");

    if let Ok(port) = port {
        port.parse().unwrap_or(DEFAULT_PORT)
    } else {
        DEFAULT_PORT
    }
}
