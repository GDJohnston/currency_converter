use std::path::PathBuf;

use crate::{cache::{self, Cache}, config};

pub(crate) mod api_key;
pub(crate) mod api_response;

const WEBSITE: &'static str = "https://v6.exchangerate-api.com/v6/";
const LATEST_PATH: &'static str = "latest/";

pub(crate) struct Api {
    key: String,
    cache: cache::Cache,
}

impl Api {
    pub(crate) fn new(config: &config::Config) -> Self {
        Api { 
            key: api_key::from_file(&config.api_key_file),
            cache: Cache::new(&config.cache)
        }
    }

    pub(crate) async fn get_rates(self, basecode: &str) -> Result<api_response::Rates, api_response::Error> {
        // Check cache for a hit
        let cache_contents = self.cache.read_from_cache(basecode);
        if cache_contents.is_some() {
            // Return cache contents to caller
            return Ok(cache_contents.unwrap())
        };

        // Request rates from website
        let rates = self.request_rates(&basecode).await?;

        // Stringify and add to cache
        let string = serde_json::to_string(&rates).unwrap();
        self.cache.add_to_cache(&basecode, &string);
        
        Ok(rates)
    }

    pub(crate) async fn request_rates(&self, basecode: &str) -> Result<api_response::Rates, api_response::Error> {
        let key = self.key.as_str();

        let url: PathBuf = [WEBSITE, key, LATEST_PATH, basecode].iter().collect();
        let url = url.to_str().unwrap();
        let response = reqwest::get(url).await.unwrap();

        api_response::parse(response).await
    }

    pub(crate) fn display_error(error: api_response::Error) {
        println!("{:#?}", error)
    }
}
