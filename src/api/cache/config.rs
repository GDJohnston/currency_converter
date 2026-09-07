//! Config for the cache module

use serde::Deserialize;
use std::path::PathBuf;

/// Config for the cache
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// Folder to store the cache data in
    pub folder: PathBuf,
    /// Override the default refresh that only requests data when new data is ready
    pub refresh_rate_secs: Option<u64>,
}
