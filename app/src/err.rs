use super::*;

#[derive(thiserror::Error, Debug)]
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