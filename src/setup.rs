use crate::config::Config;
use crate::server::RestAPIServer;
use clap::ArgMatches;

pub fn patch_config(cfg: &mut Config, matches: &ArgMatches) -> Result<(), &'static str> {
    if let Some(h) = matches.value_of("host") {
        cfg.host = h.to_string();
    }

    if let Some(p) = matches.value_of("port") {
        match p.parse::<u16>() {
            Ok(p) => cfg.port = p,
            Err(_) => return Err("failed to parse host"),
        }
    }

    Ok(())
}

pub fn setup_rest_server(cfg: &Config) -> RestAPIServer {
    RestAPIServer::new(cfg.host.parse().unwrap(), cfg.port)
}
