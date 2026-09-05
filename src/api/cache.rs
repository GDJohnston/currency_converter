use std::{
    fs::{self, File}, io::{Read, Write}, path::PathBuf, time::{Duration, SystemTime}
};
use crate::{api::api_response::Rates};
use cache_config::CacheConfig;

pub(crate) mod cache_config;

const LATEST_FOLDER: &'static str = "latest";

pub(crate) struct Cache {
    config: CacheConfig,
}

impl Cache {
    pub(crate) fn new(config: &CacheConfig) -> Self {
        Self { config: config.clone() }
    }

    pub(crate) fn read_from_cache(&self, basecode: &str) -> Option<Rates> {
        let CacheConfig { folder, refresh_rate_secs: config_refresh_rate } = &self.config;
        let cache_file = folder.join(LATEST_FOLDER).join(basecode);
        let mut file = match File::open(cache_file) {
            Ok(f) => f,
            Err(_) => return None,
        };

        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        println!("read from cache");

        let rates = Rates::new(&data);
        let Rates{time_last_update_unix,  time_next_update_unix, ..} = rates;

        let api_refresh = 
            SystemTime::UNIX_EPOCH + Duration::from_secs(time_next_update_unix as u64);
        
        let next_update = match config_refresh_rate {
            None => api_refresh,
            Some(refresh) => 
                SystemTime::UNIX_EPOCH
                + Duration::from_secs(time_last_update_unix as u64)
                + Duration::from_secs(*refresh),
        };

        let now = SystemTime::now();
        if now >= next_update {
            println!("cache expired");
            return None; // Return None so we can refresh the cache
        }

        Some(rates)
    }

    pub(crate) fn add_to_cache(&self, basecode: &str, data: &str) {
        let cache_folder = PathBuf::from(&self.config.folder).join(LATEST_FOLDER);
        let cache_file = cache_folder.join(basecode);
        fs::create_dir_all(&cache_folder).unwrap();
        let mut file = File::create(&cache_file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}
