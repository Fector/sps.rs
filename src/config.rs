use std::env;
use std::net::IpAddr;

pub struct Config {
    pub host: IpAddr,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Config {
        let default_port: u16 = 8091;
        let default_host: IpAddr = IpAddr::from([127, 0, 0, 1]);

        let host: IpAddr = match env::var("SPS_SERVER_ADDR") {
            Ok(h) => h.parse().unwrap(),
            Err(_) => default_host,
        };

        let port = match env::var("SPS_SERVER_PORT") {
            Ok(p) => p.parse::<u16>().unwrap(),
            Err(_) => default_port,
        };
        Config { host, port }
    }
}
