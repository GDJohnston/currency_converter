//! Module for processing the config file
use std::{fs, path::{Path, PathBuf}};
use serde::Deserialize;
use toml;

use super::cache::config::Config as cacheConfig;

/// Config as parsed from the config file
#[derive(Debug, Deserialize)]
pub struct Config {
    /// File holding the key for API
    pub api_key_file: PathBuf,
    /// Configuration for the cache
    pub cache: cacheConfig,
}

/// Parses the config from the config file
pub fn from(config_file: &Path) -> Config {
    let content = fs::read_to_string(config_file).unwrap();
    toml::from_str(&content).unwrap()
}
