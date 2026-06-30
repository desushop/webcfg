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

    fn index_html(&mut self, directory: camino::ReadDirUtf8) -> impl Iterator<Item=Result<camino::Utf8PathBuf, err::HtmlIndexError>> {
        directory.map(|r| {
            match r {
                Ok(d) => {
                    let path = d.path().to_owned();

                    if path.is_dir() {
                        return Err(err::HtmlIndexError::NotFile(path))
                    }

                    match path.extension() {
                        Some(s @ "html") => return Ok(path),
                        _ => return Err(err::HtmlIndexError::NotHtml(path)),
                    }
                },
                Err(e) => return Err(err::HtmlIndexError::ReadEntry(e.to_string())),
            }
        })
    }

    type Error = anyhow::Error;
    fn throw_error(&mut self, error: impl Into<Self::Error>) {
        todo!()
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