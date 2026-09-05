use std::path::PathBuf;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub folder: PathBuf,
    pub refresh_rate_secs: Option<u64>,
}
