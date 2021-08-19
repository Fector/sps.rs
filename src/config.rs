use std::env;

const DEFAULT_PORT: u16 = 8091;
const DEFAULT_HOST: &str = "127.0.0.1";

const DEFAULT_CLI_CONFIG: &str = "../configs/cli.yaml";

pub struct Config {
    pub host: String,
    pub port: u16,
    pub cli_config: String,
}

impl Config {
    pub fn from_env() -> Config {
        let host = env::var("SPS_SERVER_ADDR").unwrap_or(DEFAULT_HOST.to_string());

        let port = match env::var("SPS_SERVER_PORT") {
            Ok(p) => p.parse::<u16>().expect("failed to parse port from env"),
            Err(_) => DEFAULT_PORT,
        };

        let cli_config = env::var("SPS_CLI_CONFIG").unwrap_or(DEFAULT_CLI_CONFIG.to_string());

        Config {
            host,
            port,
            cli_config,
        }
    }
}
