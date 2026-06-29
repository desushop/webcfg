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
    fn load_html(&mut self, directory: fs::ReadDir) -> impl Iterator<Item=anyhow::Result<Self::HtmlFile>> {
        directory.map(|r| {
            match r.with_context(|| "ERR read DirEntry") {
                Ok(d) => {
                    let html_ext = std::ffi::OsStr::new("html");
                    let path = d.path();
                    match &path.extension() {
                        Some(s @ html_ext) => return compile_html(&path).with_context(|| format!("ERR compile html \"{path:?}\"")),
                        Some(_) => return anyhow::bail!("ERR file \"{path:?}\" is not an html file"),
                        None => return anyhow::bail!("ERR path \"{path:?}\" is not a file"),
                    }
                },
                Err(e) => return Err(e),
            }
        })
    }

    type Error = anyhow::Error;
    fn throw_error(&mut self, error: impl Into<Self::Error>) {
        todo!()
    }
}

fn compile_html<H>(path: impl AsRef<std::path::Path>) -> anyhow::Result<H> {
    todo!() //TODO sanitize html
}