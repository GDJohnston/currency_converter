use std::{fs, path::PathBuf};
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    pub cache_data_folder: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub website: String,
    pub key_file: PathBuf,
}

pub fn from(config_file: &str) -> Config {
    let content = fs::read_to_string(config_file).unwrap();
    toml::from_str(&content).unwrap()
}