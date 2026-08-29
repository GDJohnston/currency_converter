use std::{fs, path::PathBuf};
use serde::Deserialize;
use toml;

use crate::cache_config;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key_file: PathBuf,
    pub cache: cache_config::CacheConfig,
}

pub fn from(config_file: &str) -> Config {
    let content = fs::read_to_string(config_file).unwrap();
    toml::from_str(&content).unwrap()
}
