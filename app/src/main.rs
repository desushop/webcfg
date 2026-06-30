//! this module does not get tested,\
//! and just combines functions from library,\
//! providing a runnable default.

use std::collections;
use anyhow::Context;
use webcfg::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config().with_context(|| "ERR get config")?;
    let ip = std::net::Ipv6Addr::LOCALHOST;
    let socket = std::net::SocketAddrV6::new(ip, 35800, 0, 0);
    let mut router = axum::Router::new();
    let path = WORKING_DIR.join("sites");
    let dir = path.read_dir_utf8().with_context(|| format!("ERR read directory {path}"))?;
    let set = router.index_html(dir)
        .filter_map(|e| match e {
            Ok(p) => Some(p),
            Err(e) => {
                println!("{e:?}");
                None
            },
        }).collect::<collections::HashSet<_>>();
    router.serve(socket).await
}

fn get_config() -> anyhow::Result<webcfg::Config> {
    let path = WORKING_DIR.join("cfg.toml");
    match read_toml(&path) {
        Ok(config) => Ok(config),
        Err(e @ TomlError::Read(_, _)) => {
            let config = webcfg::Config::default();
            write_toml(config.clone(), &path)
                .with_context(|| "ERR write toml")
                .with_context(|| e)?;
            Ok(config)
        },
        Err(e) => Err(e).with_context(|| "ERR read toml")
    }
}