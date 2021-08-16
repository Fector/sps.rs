mod config;

use crate::config::{server_host, server_port};
use axum::prelude::*;
use clap::{App, Arg};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
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
            Arg::new("post")
                .short('p')
                .long("port")
                .about("Sets a custom port number.")
                .takes_value(true),
        )
        .get_matches();

    let main_fn = || async { "Simple payment system API" };
    let app = route("/", get(main_fn))
        .route(
            "/wallets",
            get(|| async { "Wallets list" }).post(|| async { "Create a new wallet" }),
        )
        .route("/wallets/:wallet", get(|| async { "Get a wallet" }));

    let host = match matches.value_of("host") {
        Some(v) => v.parse().expect("failed to parse host"),
        _ => server_host().parse().expect("failed to parse host"),
    };

    let port = match matches.value_of("port") {
        Some(v) => v.parse::<u16>().expect("failed to parse port"),
        _ => server_port(),
    };

    hyper::Server::bind(&SocketAddr::new(host, port))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
