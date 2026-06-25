use super::*;

static APP: std::sync::LazyLock<App> = std::sync::LazyLock::new(|| {
    Default::default()
});

#[tokio::test]
async fn test_serve() -> anyhow::Result<()> {
    let mut router = axum::Router::new();
    let addr = std::net::SocketAddrV6::new(std::net::Ipv6Addr::LOCALHOST, 35800, 0, 0);
    router.serve(addr).await
        .with_context(|| "ERR execute serve()");
    let req = reqwest::get(format!("http://[{}]:{}", addr.ip(), addr.port())).await
        .with_context(|| "ERR fetch request")?;

    anyhow::ensure!(req.status() == 404, "axum server is not reachable"); Ok(()) //TODO update behaviour once routes exist
}

#[tokio::test]
async fn test_load_html() -> anyhow::Result<()> {
    let mut router = axum::Router::new();
    let html = router.load_html(WORKING_DIR.deref().join("sites"))
        .with_context(|| "ERR load html files")?;
    Ok(())
}