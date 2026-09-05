use std::path::Path;

use api_response::{Error, Rates};
use cache::Cache;

pub(crate) mod api_key;
pub(crate) mod api_response;
pub(crate) mod cache;
pub(crate) mod config;

pub(crate) struct Api {
    key: String,
    cache: Cache,
}

impl Api {
    pub(crate) fn new(config_file: &Path) -> Self {
    let config = config::from(config_file);
        Api { 
            key: api_key::from_file(&config.api_key_file),
            cache: Cache::new(&config.cache)
        }
    }

    pub(crate) async fn get_rates(self, basecode: &str) -> Result<Rates, Error> {
        // Check cache for a hit
        let cache_contents = self.cache.read_from_cache(basecode);
        if cache_contents.is_some() {
            // Return cache contents to caller
            return Ok(cache_contents.unwrap())
        };

        // Request rates from website
        println!("Requesting rates from website");
        let rates = self.request_rates(&basecode).await?;

        // Stringify and add to cache
        let string = serde_json::to_string(&rates).unwrap();
        self.cache.add_to_cache(&basecode, &string);

        Ok(rates)
    }

    pub(crate) async fn request_rates(&self, basecode: &str) -> Result<Rates, Error> {
        let apikey = self.key.as_str();
        let uri = format!("https://v6.exchangerate-api.com/v6/{apikey}/latest/{basecode}");
        let response = reqwest::get(uri).await.unwrap();

        api_response::parse(response).await
    }

    pub(crate) fn display_error(error: Error) {
        println!("{:#?}", error)
    }
}
