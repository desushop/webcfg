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
    router.with_state(state).serve(socket).await
}

async fn handle_action() {
    todo!()
}

async fn serve_landing(State(app): State<sync::Arc<App>>) -> Result<response::Html<String>, (http::StatusCode, response::Html<String>)> {
    app.landing.get_or_try_init(|| {
        let p = WORKING_DIR.join("index.html");
        fs::read_to_string(p)
            .with_context(|| "ERR read index.html to string")
            .map(|s| response::Html::from(s))
    }).map(|h| h.clone()).map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, response::Html::from(format!("<html><p>{e:?}</p></html>"))))
}

async fn serve_site(State(app): State<sync::Arc<App>>, Path(site): Path<String>) -> Result<response::Html<String>, (http::StatusCode, response::Html<String>)> {
    todo!()
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