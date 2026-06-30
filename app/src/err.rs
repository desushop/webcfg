use super::*;

#[derive(thiserror::Error, Debug, Clone)]
pub enum TomlError {
    #[error("ERR read toml \"{0}\"{1}")]
    Read(camino::Utf8PathBuf, String),
    #[error("ERR deserialize toml \"{0}\"{1}")]
    Deserialize(camino::Utf8PathBuf, String),
    #[error("ERR write toml \"{0}\"{1}")]
    Write(camino::Utf8PathBuf, String),
    #[error("ERR serialize toml \"{0}\"{1}")]
    Serialize(camino::Utf8PathBuf, String),
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum HtmlIndexError {
    #[error("ERR read DirEntry: {0}")]
    ReadEntry(String),
    #[error("ERR path \"{0}\" is not html file")]
    NotHtml(camino::Utf8PathBuf),
    #[error("ERR path \"{0}\" is not a file")]
    NotFile(camino::Utf8PathBuf),
}