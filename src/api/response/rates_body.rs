//! Rates response from the API

use serde::{Serialize,Deserialize};
use serde_json::{self, Map, Value};

/// Deserialised rates response from API
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RatesBody {
    pub(crate) result: String,
    pub(crate) time_last_update_unix: u32,
    pub(crate) time_last_update_utc: String,
    pub(crate) time_next_update_unix: u32,
    pub(crate) time_next_update_utc: String,
    pub(crate) base_code: String,
    pub(crate) conversion_rates: Map<String, Value>,
}

impl RatesBody {
    /// Parse rates response from API into a struct
    pub(crate) fn new(response_body: &String) -> Self {
        serde_json::from_str(&response_body).unwrap()
    }
}
