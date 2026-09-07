//! Caching module for storing API responses so calls don't need to be repeated

use super::response::Rates;
use config::Config;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    time::{Duration, SystemTime},
};

pub(crate) mod config;

/// Cache for storing API responses
pub(crate) struct Cache {
    // Config for the cache
    config: Config,
}

impl Cache {
    /// Create a new Cache instance with provided config
    pub(crate) fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }

    /// Attempts to read from cache, returns [`Rates`] if a cache hits and is not expired
    /// else returns [`None`].
    ///
    /// Will also return None if `the last update time + the refresh rate` is less than the current time,
    /// but if the api hasn't updated then no new data will be retrieved.
    pub(crate) fn read_from_cache(&self, basecode: &str) -> Option<Rates> {
        let Config {
            folder,
            refresh_rate_secs: config_refresh_rate,
        } = &self.config;
        let cache_file = folder.join(basecode);
        let mut file = match File::open(cache_file) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        println!("read from cache");

        let rates = Rates::new(&data);
        let Rates {
            time_last_update_unix,
            time_next_update_unix,
            ..
        } = rates;

        let api_refresh =
            SystemTime::UNIX_EPOCH + Duration::from_secs(time_next_update_unix as u64);

        let next_update = match config_refresh_rate {
            None => api_refresh,
            Some(refresh) => {
                SystemTime::UNIX_EPOCH
                    + Duration::from_secs(time_last_update_unix as u64)
                    + Duration::from_secs(*refresh)
            }
        };

        let now = SystemTime::now();
        if now >= next_update {
            println!("cache expired");
            return None; // Return None so we can refresh the cache
        }

        Some(rates)
    }

    /// Adds an entry into the cache.
    ///
    /// Overwrites an exising entry if it exists.
    pub(crate) fn add_to_cache(&self, basecode: &str, data: &str) {
        let cache_folder = PathBuf::from(&self.config.folder);
        let cache_file = cache_folder.join(basecode);
        fs::create_dir_all(&cache_folder).unwrap();

        let mut file = File::create(&cache_file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}
