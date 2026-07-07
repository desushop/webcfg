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

pub fn sanitize_html(html: impl AsRef<str>, tags: HashSet<String>, attr: HashMap<String, HashSet<String>>) -> String {
    ammonia::Builder::empty()
        .tags(tags.iter().map(|s| s.as_str()).collect())
        .tag_attributes(attr.iter().map(|(s, e)| (s.as_str(), e.into_iter().map(|s| s.as_str()).collect())).collect())
        .clean(html.as_ref())
        .to_string()
}

type HtmlFile = String;
fn compile_html(path: impl AsRef<std::path::Path>, tags: HashSet<String>, attr: HashMap<String, HashSet<String>>) -> anyhow::Result<HtmlFile> {
    let html = fs::read_to_string(path)
        .with_context(|| format!("ERR read file to string"))?;
    Ok(sanitize_html(html, tags, attr))
}