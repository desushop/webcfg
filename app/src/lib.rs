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

async fn serve<S>(socket: S, app: axum::Router) -> anyhow::Result<()>
where S: tokio::net::ToSocketAddrs + std::fmt::Debug {
    let listener = tokio::net::TcpListener::bind(&socket).await
        .with_context(|| format!("failed to create socket from {socket:?}"))?;
    axum::serve(listener, app).await
        .with_context(|| "failed to start axum server")
        .map(|_| ())
}

#[tokio::test]
async fn test_serve() -> anyhow::Result<()> {
    serve("::1", axum::Router::new()).await
        .with_context(|| "failed to execute serve()")
}