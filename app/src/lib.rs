//! # webcfg
//! ## goal
//! a webserver which allows anyone with a steam account to create and upload a website.
//! ## features
//! - steam openid auth
//! - user custom html
//! - steam user profile integration
//! ## guidelines
//! this project will follow test driven development.\
//! in order to retain motivation and prevent burnout,
//! the challenge is to write atleast one function with one test a day.\
//! this includes rewriting and refactoring.
//!
//! - write function signature
//! - write unit test defining and proving a functions contract
//! - write function
//! ## license
//! © single license - AGPL-3.0 - maoperson@desushop

use anyhow::Context;

fn serve<S>(socket: S, app: axum::Router) -> anyhow::Result<()>
where for<'a> S: 'a + Send + Sync + tokio::net::ToSocketAddrs + std::fmt::Debug {
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(&socket).await
            .with_context(|| format!("ERR create socket from {socket:?}"))?;
        axum::serve(listener, app).await
            .with_context(|| "ERR start axum server")
            .map(|_| ())
    }); Ok(())
}

async fn test_serve() -> anyhow::Result<()> {
    let addr = "::1:35800";
    serve(addr, axum::Router::new())
        .with_context(|| "ERR execute serve()");
    let req = reqwest::get(format!("http://{addr}")).await
        .with_context(|| "ERR fetch request")?;

    anyhow::ensure!(req.status() == 404, "axum server is not reachable"); Ok(())
}