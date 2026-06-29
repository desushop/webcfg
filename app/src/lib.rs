//! # webcfg
//! ## goal
//! a webserver which allows anyone with a steam account to create and upload a website.
//! ## features
//! - steam openid auth
//! - user custom html
//! - steam user profile integration
//! ## guidelines
//! this project will follow test driven development.\
//! in order to retain motivation and prevent burnout,
//! the challenge is to write atleast one function with one test a day.\
//! this includes rewriting and refactoring.
//!
//! - write function signature
//! - write unit test defining and proving a functions contract
//! - write function
//! ## license
//! © single license - AGPL-3.0 - maoperson@desushop

pub use router::*;
pub use err::*;

use std::{fs, net, ops::Deref};
use anyhow::Context;
use std::fmt::Debug;

mod err;
mod router;
#[cfg(test)]
mod test;

pub static WORKING_DIR: std::sync::LazyLock<camino::Utf8PathBuf> = std::sync::LazyLock::new(|| {
    let exe = std::env::current_exe().expect("ERR read working directory");
    let utf = camino::Utf8PathBuf::from_path_buf(exe).expect("ERR convert utf8 path");
    return utf.with_file_name("")
});

/// axum app state
#[derive(Debug, Default)]
pub struct App {
    error_bucket: Vec<anyhow::Error>
}

/// toml config which controls server parameters
#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
pub struct Config {
    ip: net::IpAddr,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: net::IpAddr::V6(net::Ipv6Addr::LOCALHOST),
            port: 35800,
        }
    }
}

pub trait WebcfgRunnable {
    #[allow(async_fn_in_trait)]
    async fn serve<S>(self, socket: S) -> anyhow::Result<()>
    where for<'a> S: 'a + Send + Sync + tokio::net::ToSocketAddrs + std::fmt::Debug;

    type HtmlFile;
    /// loads html files from a target directory
    fn load_html(&mut self, directory: fs::ReadDir) -> impl Iterator<Item=anyhow::Result<Self::HtmlFile>>;

    type Error;
    fn throw_error(&mut self, error: impl Into<Self::Error>);
}

pub fn read_toml(target: impl AsRef<camino::Utf8Path>) -> Result<Config, err::TomlError> {
    let target = target.as_ref();
    let bytes = std::fs::read(target)
        .map_err(|e| err::TomlError::Read(target.to_path_buf(), format!(": {e}")))?;
    toml::from_slice(bytes.as_slice())
        .map_err(|e| err::TomlError::Deserialize(target.to_path_buf(), format!(": {e}")))
}

pub fn write_toml<C>(contents: C, target: impl AsRef<camino::Utf8Path>) -> Result<impl AsRef<camino::Utf8Path>, err::TomlError>
where for<'a> C: 'a + serde::Serialize + Debug {
    let bytes = toml::to_string_pretty(&contents)
        .map_err(|e| err::TomlError::Serialize(target.as_ref().to_path_buf(), format!(": {e}")))?;
    fs::write(&target.as_ref(), bytes)
        .map_err(|e| err::TomlError::Write(target.as_ref().to_path_buf(), format!(": {e}")))
        .map(|_| target)
}