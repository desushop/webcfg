use std::ops::Deref;

use anyhow::Context;
use webcfg::*;

static APP: std::sync::LazyLock<App> = std::sync::LazyLock::new(|| {
    Default::default()
});

#[tokio::test]
async fn test_serve() -> anyhow::Result<()> {
    let mut router = axum::Router::new();
    let addr = "::1:35800";
    router.serve(addr)
        .with_context(|| "ERR execute serve()");
    let req = reqwest::get(format!("http://{addr}")).await
        .with_context(|| "ERR fetch request")?;

    anyhow::ensure!(req.status() == 404, "axum server is not reachable"); Ok(())
}

#[tokio::test]
async fn test_load_html() -> anyhow::Result<()> {
    let mut router = axum::Router::new();
    router.load_html(WORKING_DIR.deref().join("sites"))
        .with_context(|| "ERR load html files");
    Ok(())
}