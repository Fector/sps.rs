mod config;
mod server;
mod setup;

use crate::config::Config;
use crate::setup::{patch_config, setup_rest_server};

use clap::{load_yaml, App, AppSettings};

#[tokio::main]
async fn main() {
    let mut cfg = Config::from_env();

    let yaml = load_yaml!("../configs/cli.yaml");
    let matches = App::from(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    // start web server if "run" subcommand exists
    if let Some(m) = matches.subcommand_matches("run") {
        patch_config(&mut cfg, m).unwrap();

        println!("Running server on {}:{}", cfg.host, cfg.port);

        setup_rest_server(&cfg).start().await;
    }
}
