use axum::prelude::*;
use std::net::IpAddr;
use std::net::SocketAddr;

pub struct RestAPIServer {
    host: IpAddr,
    port: u16,
}

impl RestAPIServer {
    pub fn new(host: IpAddr, port: u16) -> RestAPIServer {
        RestAPIServer { host, port }
    }

    pub async fn start(&self) {
        let main_fn = || async { "Simple payment system API" };
        let app = route("/", get(main_fn))
            .route(
                "/wallets",
                get(|| async { "Wallets list" }).post(|| async { "Create a new wallet" }),
            )
            .route("/wallets/:wallet", get(|| async { "Get a wallet" }));

        hyper::Server::bind(&SocketAddr::new(self.host, self.port))
            .serve(app.into_make_service())
            .await
            .unwrap()
    }
}
