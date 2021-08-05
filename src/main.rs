use axum::prelude::*;

#[tokio::main]
async fn main() {
    let main_fn = || async { "Simple payment system API" };
    let app = route("/", get(main_fn))
        .route(
            "/wallets",
            get(|| async { "Wallets list" }).post(|| async { "Create a new wallet" }),
        )
        .route("/wallets/:wallet", get(|| async { "Get a wallet" }));

    hyper::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
