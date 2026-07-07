use std::{default, str::FromStr};

use super::*;

static APP: std::sync::LazyLock<App> = std::sync::LazyLock::new(|| {
    Default::default()
});
static BUILD_DIR: std::sync::LazyLock<camino::Utf8PathBuf> = std::sync::LazyLock::new(|| {
    camino::Utf8PathBuf::from_str(env!("CARGO_MANIFEST_DIR")).expect("ERR convert utf8 path")
        .parent()
        .unwrap()
        .join(".build")
});

#[tokio::test]
async fn test_serve() -> anyhow::Result<()> {
    let mut router = axum::Router::new();
    let addr = std::net::SocketAddrV6::new(std::net::Ipv6Addr::LOCALHOST, 35800, 0, 0);
    axum::Router::<sync::Arc<App>>::serve(router, addr).await
        .with_context(|| "ERR execute serve()");
    let req = reqwest::get(format!("http://[{}]:{}", addr.ip(), addr.port())).await
        .with_context(|| "ERR fetch request")?;

    anyhow::ensure!(req.status() == 404, "axum server is not reachable"); Ok(()) //TODO update behaviour once routes exist
}

#[tokio::test]
async fn test_index_html() -> anyhow::Result<()> {
    let path = BUILD_DIR.deref().join("sites");
    let dir = path.read_dir_utf8()
        .with_context(|| format!("ERR read directory {path}"))?;
    let mut failures = 0u16;
    let mut failures2 = 0u16;
    let mut directories = 0u16;
    let mut directories2 = 0u16;
    let mut misc_files = 0u16;
    let mut misc_files2 = 0u16;
    let mut successes = 0u16;
    let mut successes2 = 0u16;
    let entries = dir.for_each(|e| {
        match e {
            Ok(e) => {
                let p = e.into_path();
                if p.is_dir() {
                    directories += 1;
                    return;
                }
                match p.extension() {
                    Some(s @ "html") => successes += 1,
                    _ => misc_files += 1,
                } return;
            },
            Err(_) => failures += 1,
        }
    });
    IndexBuilder::new(path.as_path()).build()
        .with_context(|| "ERR build html index")?
        .for_each(|(_, site)| {
            if let Site::Err(e) = site {
                match e {
                    HtmlIndexError::ReadEntry(..) => failures2 += 1,
                    HtmlIndexError::NotFile(..) => directories2 += 1,
                    HtmlIndexError::NotHtml(..) => misc_files2 += 1,
                }
            } else {
                successes2 += 1;
            }
        });

    assert!(failures2 == failures);
    assert!(directories2 == directories);
    assert!(misc_files == misc_files2);
    assert!(successes == successes2);
    Ok(())
}

#[test]
fn test_write_toml() -> anyhow::Result<()> {
    let path = BUILD_DIR.join("cfg.toml");
    let out = write_toml(Config::default(), &path).with_context(|| "ERR write toml")?;
    anyhow::ensure!(path.as_path() == out.as_ref(), "paths did not match");
    Ok(())
}

#[test]
fn test_read_toml() -> anyhow::Result<()> {
    test_write_toml()?; //NOTE just stabilize like this; too lazy to fix test ordering
    let config = Config::default();
    let out = read_toml(BUILD_DIR.join("cfg.toml")).with_context(|| "ERR read toml")?;
    anyhow::ensure!(config == out, "configs did not match");
    Ok(())
}

#[test]
fn test_sanitize_html() -> anyhow::Result<()> {
    let cfg = Config::default();
    let _in = "<html><link src=\"./index.css\"/><script><test/></script><style>* { opacity: 0.0; }</style></html>".to_owned();
    let out = sanitize_html(&_in, cfg.tags, cfg.attr);
    anyhow::ensure!(_in != out, "html did not sanitize");
    anyhow::ensure!(out == "<html><style>* { opacity: 0.0; }</style></html>".to_owned(), "html did not sanitize correctly");
    Ok(())
}