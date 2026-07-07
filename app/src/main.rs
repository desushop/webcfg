//! this module does not get tested,\
//! and just combines functions from library,\
//! providing a runnable default.

use std::collections;
use anyhow::Context;
use axum::extract::State;
use webcfg::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = get_config().with_context(|| "ERR get config")?;
    let ip = std::net::Ipv6Addr::LOCALHOST;
    let socket = std::net::SocketAddrV6::new(ip, 35800, 0, 0);
    let mut router = axum::Router::<App>::new();
    let path = WORKING_DIR.join("sites");
    let index = IndexBuilder::new(path.as_path()).build().with_context(|| "ERR build html index")?;
    let state = App::new(index.map(|(n, s)| { println!("{s:?}"); (n,s) }));
    axum::Router::<App>::serve(router.with_state(state), socket).await.with_context(|| "ERR serve axum")?;
    loop {}
}

//TODO handle site loading errors dynamically, by compiling error htmls from templates of dylib via askama

//TODO handle openid and stored auth keys loading
// user access page -> check if have auth key ? route to homepage : route to openid signin to store auth key
//

//TODO dynamically load dylibs and match /{site}/{action} to no_mangle fn from dylib
//async fn handle_action(Path((site, action)): Path<(String, String)>, Query(): Query<>) { //TODO return type and handle response on html without js
//
//}

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