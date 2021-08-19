use crate::config::Config;
use clap::ArgMatches;
use std::net::IpAddr;

pub fn patch_config(cfg: &mut Config, matches: &ArgMatches) -> Result<(), &'static str> {
    if let Some(h) = matches.value_of("host") {
        match h.parse::<IpAddr>() {
            Ok(h) => cfg.host = h,
            Err(_) => return Err("failed to parse host"),
        }
    }

    if let Some(p) = matches.value_of("port") {
        match p.parse::<u16>() {
            Ok(p) => cfg.port = p,
            Err(_) => return Err("failed to parse host"),
        }
    }

    Ok(())
}
