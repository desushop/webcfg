use anyhow::anyhow;

use super::*;

impl<S> crate::WebcfgRunnable for axum::Router<S> {
    async fn serve(router: axum::Router, socket: impl tokio::net::ToSocketAddrs + std::fmt::Debug) -> anyhow::Result<()> {
        let listener = tokio::net::TcpListener::bind(&socket).await
            .with_context(|| format!("ERR create socket from {socket:?}"))?;
        tokio::spawn(async move {
            axum::serve(listener, router)
                //.with_graceful_shutdown(signal) // TODO handle intentional shutdowns to prevent panics
                .await
                .with_context(|| "ERR start axum server")
                .unwrap()
        }); Ok(())
    }
}

type HtmlFile = ();
fn compile_html(path: impl AsRef<std::path::Path>, tags: &[impl std::borrow::Borrow<str>]) -> anyhow::Result<HtmlFile> {
    let html = fs::read_to_string(path)
        .with_context(|| format!("ERR read file to string"))?;
    ammonia::Builder::empty()
        .add_tags(tags);
    todo!()
}