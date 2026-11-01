//! API-based currency converter.
//!
//! This tool convers tool converts between any two [supported currencies], using an api call.
//!
//! To use this tool, you need an api key from the [ExchangeRate-API] website.
//! This tool uses a caching system that only calls the api if the required data is missing or expired,
//! to reduce request quota usage.
//! 
//! [ExchangeRate-API] is a product of AYR Tech (Pty) Ltd.
//! 
//! 
//! [ExchangeRate-API]: https://www.exchangerate-api.com/
//! [supported currencies]: https://www.exchangerate-api.com/docs/supported-currencies
//! 
//! ```bash
//! $currency_converter
//! Usage: currency_converter <basecode> [targetcode] [units]
//! ```
//! 
//! # Examples
//! 
//! For basic usage supply a `basecode` and a `targetcode`:
//! 
//! ```bash
//! $ currency_converter USD GBP
//! 1 USD = 0.749 GBP
//! ```
//! 
//! If `units` is provided, the exchange is multiplied by that value:
//! 
//! ```bash
//! $ currency_converter USD GBP 1.5
//! 1.5 USD = 1.12035 GBP
//! ```
//! 
//! If `targetcode` is omitted, the list of all exchange rates is returned:
//! 
//! ```bash
//! $ currency_converter USD
//! "USD rates": {
//! ...
//!   "GBP": 0.7469,
//! ...
//! }
//! ```

use std::path::Path;

use crate::{api::Api, cli::Args};

mod api;
mod cli;

/// File storing the config settings
const CONFIG_FILE: &'static str = "./Config.toml";

#[tokio::main]
async fn main() {
    // Create a new Api object with the config found in the config file
    let config_path = Path::new(CONFIG_FILE);
    let api = Api::new(&config_path);

    // Pull arguments from the command line
    let Args {
        base,
        target,
        units,
    } = Args::new();

    // Get rates for the basecode
    let basecode = base.to_uppercase();
    let response = api.get_rates(&basecode).await;
    let rates = match response {
        Ok(r) => r,
        Err(e) => {
            Api::display_error(e);
            return;
        }
    };

    let targetcode = target.to_uppercase();
    if targetcode.is_empty() {
        println!(
            "\"{} rates\": {}",
            basecode,
            serde_json::to_string_pretty(&rates.conversion_rates).unwrap()
        );
        return;
    }

    if rates.conversion_rates.contains_key(&targetcode) {
        let conversion_rate = rates.conversion_rates[&targetcode].as_f64().unwrap();
        let converted_units = conversion_rate * units;
        println!("{units} {basecode} = {converted_units} {targetcode}");
    } else {
        eprintln!("Unknown targetcode: {targetcode}");
    }
}
