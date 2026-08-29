use crate::{api::Api, cli::Args};

mod cache_config;
mod config;
mod cache;
mod cli;
mod api;

const CONFIG_FILE: &'static str = "./Config.toml";

#[tokio::main]
async fn main() {
    let config = config::from(CONFIG_FILE);
    let Args{ base, target, units } = cli::Args::new();
    let api = api::Api::new(&config);

    let basecode = base.to_uppercase();
    let response = api.get_rates(&basecode).await;
    let rates = match response {
        Ok(r) => r,
        Err(e) => {Api::display_error(e); return;},
    };

    let targetcode = target.to_uppercase();
    if targetcode.is_empty() {
        println!("{}", serde_json::to_string_pretty(&rates.conversion_rates).unwrap());
        return;
    }

    if rates.conversion_rates.contains_key(&targetcode)
    {
        let conversion_rate = rates.conversion_rates[&targetcode].as_f64().unwrap();
        let converted_units = conversion_rate * units;
        println!("{units} {basecode} = {converted_units} {targetcode}");
    }
}
