mod config;
mod server;
mod setup;

use crate::config::Config;
use crate::setup::setup_rest_server;

use clap::{load_yaml, App, AppSettings};

#[tokio::main]
async fn main() {
    let cfg = Config::from_env();

    let yaml = load_yaml!("../configs/cli.yaml");
    let matches = App::from(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // start web server if "run" subcommand exists
    if let Some(m) = matches.subcommand_matches("run") {
        let mut host = cfg.host;
        let mut port = cfg.port;

        // check host arg
        if let Some(h) = m.value_of("host") {
            host = h.to_string();
        }

        // check port arg
        if let Some(p) = m.value_of("port") {
            port = p.parse::<u16>().expect("failed to parse host")
        }

        println!("Running server on {}:{}", host, port);

        setup_rest_server(host, port).start().await;
    }
}
