use anyhow::anyhow;

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
    fn load_html(&mut self, directory: fs::ReadDir) -> impl Iterator<Item=Self::HtmlFile> {
        directory
            .filter_map(|r| drop_failures(self, r.with_context(|| "ERR read DirEntry")))
            .filter(|d| d.path().extension() == )
            .map(|d| compile_html(d.path()))
            .filter_map(|r| drop_failures(self, r.with_context(|| "ERR compile html")))
    }

    type Error = anyhow::Error;
    fn throw_error(&mut self, error: impl Into<Self::Error>) {
        todo!()
    }
}

fn drop_failures<T>(router: &mut axum::Router, result: anyhow::Result<T>) -> Option<T> {
    match result {
        Err(e) => { router.throw_error(e); None }
        Ok(d) => Some(d)
    }
}

fn compile_html<H>(path: impl AsRef<std::path::Path>) -> anyhow::Result<H> {
    todo!() //TODO sanitize html
}