use super::*;

impl crate::WebcfgRunnable for axum::Router {
    fn serve<S>(self, socket: S) -> anyhow::Result<()>
    where for<'a> S: 'a + Send + Sync + tokio::net::ToSocketAddrs + std::fmt::Debug {
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(&socket).await
                .with_context(|| format!("ERR create socket from {socket:?}"))?;
            axum::serve(listener, self).await
                .with_context(|| "ERR start axum server")
                .map(|_| ())
        }); Ok(())
    }

    type HtmlFile = ();
    fn load_html<'a>(&mut self, target: impl AsRef<camino::Utf8Path>) -> anyhow::Result<&'a [Self::HtmlFile]> {
        todo!()
    }
}