use crate::server::RestAPIServer;

pub fn setup_rest_server(host: String, port: u16) -> RestAPIServer {
    RestAPIServer::new(
        host.parse().expect("failed to parse http server host addr"),
        port,
    )
}
