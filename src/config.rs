use std::fs;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
    pub _data: Option<DataConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub key : String,
}

#[derive(Debug, Deserialize)]
pub struct DataConfig {
    pub _refresh_secs: u32,
}

pub fn get_config(config_file: &str) -> Config {
    let content = fs::read_to_string(config_file).unwrap();
    toml::from_str(&content).unwrap()
}