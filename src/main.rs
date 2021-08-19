mod config;
mod setup;

use crate::config::Config;
use crate::setup::patch_config;
use axum::prelude::*;
use clap::{load_yaml, App, AppSettings};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let mut cfg = Config::from_env();

    let main_fn = || async { "Simple payment system API" };
    let app = route("/", get(main_fn))
        .route(
            "/wallets",
            get(|| async { "Wallets list" }).post(|| async { "Create a new wallet" }),
        )
        .route("/wallets/:wallet", get(|| async { "Get a wallet" }));

    let yaml = load_yaml!("../configs/cli.yaml");
    let matches = App::from(yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(m) = matches.subcommand_matches("run") {
        patch_config(&mut cfg, m).unwrap();

        println!("Running server on {}:{}", cfg.host, cfg.port);

        hyper::Server::bind(&SocketAddr::new(cfg.host, cfg.port))
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
