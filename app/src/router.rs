use super::*;

impl crate::WebcfgRunnable for axum::Router {
    async fn serve<S>(self, socket: S) -> anyhow::Result<()>
    where for<'a> S: 'a + Send + Sync + tokio::net::ToSocketAddrs + std::fmt::Debug {
        let listener = tokio::net::TcpListener::bind(&socket).await
            .with_context(|| format!("ERR create socket from {socket:?}"))?;
        tokio::spawn(async move {
            axum::serve(listener, self)
                //.with_graceful_shutdown(signal) // TODO handle intentional shutdowns to prevent panics
                .await
                .with_context(|| "ERR start axum server")
                .unwrap()
        }); Ok(())
    }

    type HtmlFile = ();
    fn load_html<'a>(&mut self, target: impl AsRef<camino::Utf8Path>) -> anyhow::Result<&'a [Self::HtmlFile]> {
        todo!()
    }
}