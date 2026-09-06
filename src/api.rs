//! Module for requesting exchange rates and caching the result

use std::path::Path;

use response::{Error, Rates};
use cache::Cache;

pub(crate) mod key;
pub(crate) mod response;
pub(crate) mod cache;
pub(crate) mod config;

/// Holds the api key and the cache for api responses
pub(crate) struct Api {
    /// Key for the api
    key: String,
    /// Cache for the api results
    cache: Cache,
}

impl Api {
    /// Generate a API instance with a configuration from the configuration file
    pub(crate) fn new(config_file: &Path) -> Self {
    let config = config::from(config_file);
        Api { 
            key: key::from_file(&config.api_key_file),
            cache: Cache::new(&config.cache)
        }
    }

    /// Get conversion rates.
    /// 
    /// Checks the cache for a valid response and returns if there is a hit,
    /// otherwise makes an api call and returns that result.
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

    /// Request the exchange rates for the base currency
    pub(crate) async fn request_rates(&self, basecode: &str) -> Result<Rates, Error> {
        let apikey = self.key.as_str();
        let uri = format!("https://v6.exchangerate-api.com/v6/{apikey}/latest/{basecode}");
        let response = reqwest::get(uri).await.unwrap();

        response::parse(response).await
    }

    /// Formats and displays an error from the website
    pub(crate) fn display_error(error: Error) {
        println!("{:#?}", error)
    }
}
