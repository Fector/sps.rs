use std::env;

pub fn server_address() -> String {
    env::var("SERVER_ADDR").unwrap_or("127.0.0.1".to_string())
}

pub fn server_port() -> u16 {
    let default_port: u16 = 8080;
    let env_value = env::var("SERVER_PORT");
    if let Ok(port) = env_value {
        return port.parse::<u16>().unwrap_or(default_port);
    }
    default_port
}
