//! API-based currency converter.
//!
//! Using AYR Tech (Pty) Ltd's [exchangerate-api](https://www.exchangerate-api.com/),
//! this tool converts from any currency [supported by the api](https://www.exchangerate-api.com/docs/supported-currencies)
//! to any other supported currency.
//! The tool also multiplies the conversion if the `units` argument is supplied.
//! If the `targetcode` argument is not supplied, the tool will report all supported rates for the base currency.
//!
//! This tool uses a caching system that only calls the api if the required data is missing or expired,
//! saving on the request quota.
//!
//! To use this tool, you need an api key from the exchangerate-api website.

use std::path::Path;

use crate::{api::Api, cli::Args};

mod api;
mod cli;

/// File storing the config settings
const CONFIG_FILE: &'static str = "./Config.toml";

#[tokio::main]
async fn main() {
    let config_path = Path::new(CONFIG_FILE);
    let api = Api::new(&config_path);

    let Args {
        base,
        target,
        units,
    } = Args::new();

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
            "{}",
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
