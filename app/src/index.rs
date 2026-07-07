use super::*;

#[derive(Clone, Debug)]
pub enum Site<E> {
    Html(String),
    Path(camino::Utf8PathBuf),
    Err(E),
}

#[derive(Clone, Debug)]
pub struct IndexBuilder<'a> {
    dir: &'a camino::Utf8Path
}

impl<'a> IndexBuilder<'a> {
    pub fn new(dir: impl Into<&'a camino::Utf8Path>) -> Self {
        Self {
            dir: dir.into()
        }
    }

    pub fn build(self) -> anyhow::Result<impl Iterator<Item=(String, Site<HtmlIndexError>)>> {
        let r = self.dir
            .read_dir_utf8()
            .with_context(|| "ERR read dir as utf8")?
            .map(|r| r.with_context(|| format!("ERR walk dir {}", self.dir)))
            .map(|r| match r {
                Ok(e) => Self::get_prefix(e.into_path()),
                Err(e) => Err(e),
            }).collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .map(|(name, path)| {
                match Self::evaluate_path(&path) {
                    Ok(_) => (name, Site::Path(path)),
                    Err(e) => (name, Site::Err(e)),
                }
            });
        Ok(r)
    }

    fn evaluate_path(path: &camino::Utf8Path) -> Result<(), HtmlIndexError> {
        if path.is_dir() {
            return Err(HtmlIndexError::NotFile(path.to_owned()))
        }

        match path.extension() {
            Some(s @ "html") => return Ok(()),
            _ => return Err(HtmlIndexError::NotHtml(path.to_owned())),
        }
    }

    fn get_prefix(path: camino::Utf8PathBuf) -> anyhow::Result<(String, camino::Utf8PathBuf)> {
        let name = path.file_prefix()
            .with_context(|| format!("ERR file prefix of \"{path}\" is None"))?
            .to_owned();
        Ok((name, path))
    }
}