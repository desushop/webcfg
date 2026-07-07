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
pub use index::*;
pub use err::*;

use std::{collections, fs, net, ops::Deref, sync};
use anyhow::Context;
use std::fmt::Debug;

mod err;
mod router;
mod index;
#[cfg(test)]
mod test;

pub static WORKING_DIR: std::sync::LazyLock<camino::Utf8PathBuf> = std::sync::LazyLock::new(|| {
    let exe = std::env::current_exe().expect("ERR read working directory");
    let utf = camino::Utf8PathBuf::from_path_buf(exe).expect("ERR convert utf8 path");
    return utf.with_file_name("")
});

/// axum app state
#[derive(Debug, Default, Clone)]
pub struct App {
    html_index: ahash::HashMap<String, Site<HtmlIndexError>>
}

impl App {
    pub fn new(iter: impl Iterator<Item=(String, Site<HtmlIndexError>)>) -> Self {
        Self {
            html_index: iter.collect()
        }
    }
}

/// toml config which controls server parameters
#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
pub struct Config {
    ip: net::IpAddr,
    port: u16,
    tags: HashSet<String>,
    attr: HashMap<String, HashSet<String>>
}

impl Default for Config {
    fn default() -> Self {
        let default_tags = &[
            "class", "draggable", "dir", "title", "role",
            "aria-checked", "aria-autocomplete", "aria-disabled", "aria-label", "aria-valuemax", "aria-valuemin", "aria-valuenow", "aria-valuetext", "aria-placeholder",
            "aria-dropeffect", "aria-dragged",
            "aria-busy", "aria-description", "aria-details", "aria-label", "aria-keyshortcuts", "value", "name"];
        Self {
            ip: net::IpAddr::V6(net::Ipv6Addr::LOCALHOST),
            port: 35800,
            tags: HashSet::from_iter([
                "div", "span", "style", "a", "body", "button", "br", "code", "details", "head", "footer", "html", "img", "input", "i", "p", "svg", "textarea", "title", "option", "form"
            ].into_iter().map(String::from)),
            attr: HashMap::from_iter([
                ("img".to_owned(), HashSet::from_iter(["src", "href", "alt"].into_iter().chain(default_tags.clone()).map(String::from))),
                ("input".to_owned(), HashSet::from_iter(["alt", "placeholder", "readonly", "required", "list", "disabled", "type", "src"].into_iter().chain(default_tags.clone()).map(String::from))),
                ("html".to_owned(), HashSet::from_iter(["lang"].into_iter().map(String::from))),
                ("a".to_owned(), HashSet::from_iter(["href"].into_iter().map(String::from))),
                ("textarea".to_owned(), HashSet::from_iter(["wrap", "rows", "required", "readonly", "placeholder", "disabled"].into_iter().chain(default_tags.clone()).map(String::from))),
                ("button".to_owned(), HashSet::from_iter(["disabled"].into_iter().chain(default_tags.clone()).map(String::from))),
                ("option".to_owned(), HashSet::from_iter(["disabled"].into_iter().map(String::from))),
                ("form".to_owned(), HashSet::from_iter(["action", "method"].into_iter().map(String::from))),
            ].into_iter())
        }
    }
}

pub trait WebcfgRunnable {
    #[allow(async_fn_in_trait)]
    async fn serve<S>(self, socket: S) -> anyhow::Result<()>
    where for<'a> S: 'a + Send + Sync + tokio::net::ToSocketAddrs + std::fmt::Debug;
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