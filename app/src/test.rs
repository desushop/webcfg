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
    let path = WORKING_DIR.deref().join("sites");
    let dir = std::fs::read_dir(&path)
        .with_context(|| format!("ERR read directory {}", &path))?;
    let html = router.load_html(dir, );
    let html = html.collect::<anyhow::Result<Vec<_>>>()?;
    Ok(())
}

#[test]
fn test_write_toml() -> anyhow::Result<()> {
    let path = WORKING_DIR.join("cfg.toml");
    let out = write_toml(Config::default(), &path).with_context(|| "ERR write toml")?;
    anyhow::ensure!(path.as_path() == out.as_ref(), "paths did not match");
    Ok(())
}

#[test]
fn test_read_toml() -> anyhow::Result<()> {
    let config = Config::default();
    let out = read_toml(WORKING_DIR.join("cfg.toml")).with_context(|| "ERR read toml")?;
    anyhow::ensure!(config == out, "configs did not match");
    Ok(())
}