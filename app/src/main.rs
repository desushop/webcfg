//! this module does not get tested,\
//! and just combines functions from library,\
//! providing a runnable default.

use anyhow::Context;
use webcfg::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config().with_context(|| "ERR get config")?;

    let ip = std::net::Ipv6Addr::LOCALHOST;
    let socket = std::net::SocketAddrV6::new(ip, 35800, 0, 0);
    axum::Router::new().serve(socket).await
}

fn get_config() -> anyhow::Result<webcfg::Config> {
    let path = WORKING_DIR.join("cfg.toml");
    match read_toml(&path).with_context(|| "ERR read toml") {
        Ok(config) => Ok(config),
        Err(re) => {
            let config = webcfg::Config::default();
            write_toml(config.clone(), &path)
                .with_context(|| "ERR write toml")
                .with_context(|| re)?;
            Ok(config)
        },
    }
}