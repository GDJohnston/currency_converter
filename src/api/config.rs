use std::{fs, path::{Path, PathBuf}};
use serde::Deserialize;
use toml;

use super::cache::cache_config::CacheConfig;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key_file: PathBuf,
    pub cache: CacheConfig,
}

pub fn from(config_file: &Path) -> Config {
    let content = fs::read_to_string(config_file).unwrap();
    toml::from_str(&content).unwrap()
}
