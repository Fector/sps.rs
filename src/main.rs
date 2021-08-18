mod config;
mod setup;

use crate::config::Config;
use crate::setup::patch_config;
use axum::prelude::*;
use clap::{App, Arg};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let mut cfg = Config::from_env();
    let matches = App::new("Simple payment system")
        .version("0.0.1")
        .about("Pet project with Axum")
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .about("Sets a custom host address.")
                .takes_value(true),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .about("Sets a custom port number.")
                .takes_value(true),
        )
        .get_matches();

    patch_config(&mut cfg, matches).unwrap();

    let main_fn = || async { "Simple payment system API" };
    let app = route("/", get(main_fn))
        .route(
            "/wallets",
            get(|| async { "Wallets list" }).post(|| async { "Create a new wallet" }),
        )
        .route("/wallets/:wallet", get(|| async { "Get a wallet" }));

    println!("Running server on {}:{}", cfg.host, cfg.port);

    hyper::Server::bind(&SocketAddr::new(cfg.host, cfg.port))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
