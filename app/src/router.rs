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
        let target = target.as_ref();
        std::fs::read_dir(target)
            .with_context(|| format!("ERR read directory {}", target))?
            .filter_map(|r| match r {
                Ok(d) => {
                    if let Ok(path) = camino::Utf8PathBuf::from_path_buf(d.path()) {
                        if let Ok(html) = compile_html::<Self::HtmlFile>(path) {
                            return Some(html)
                        } return None
                    } return None
                },
                Err(e) => {
                    self.throw_error(anyhow::anyhow!(e));
                    return None
                },
            }); todo!("this fn is trash, do better and propagate")
    } //TODO return iter

    type Error = anyhow::Error;
    fn throw_error(&mut self, error: impl Into<Self::Error>) {
        todo!()
    }
}