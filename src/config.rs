use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Config {
        let default_port: u16 = 8091;
        let default_host: String = String::from("127.0.0.1");

        let host = match env::var("SPS_SERVER_ADDR") {
            Ok(h) => h,
            Err(_) => default_host,
        };

        let port = match env::var("SPS_SERVER_PORT") {
            Ok(p) => p.parse::<u16>().unwrap(),
            Err(_) => default_port,
        };
        Config { host, port }
    }
}
