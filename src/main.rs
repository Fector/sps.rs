mod config;

use crate::config::{server_address, server_port};
use axum::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let main_fn = || async { "Simple payment system API" };
    let app = route("/", get(main_fn))
        .route(
            "/wallets",
            get(|| async { "Wallets list" }).post(|| async { "Create a new wallet" }),
        )
        .route("/wallets/:wallet", get(|| async { "Get a wallet" }));

    hyper::Server::bind(&SocketAddr::new(
        server_address().parse().unwrap(),
        server_port(),
    ))
    .serve(app.into_make_service())
    .await
    .unwrap();
}
