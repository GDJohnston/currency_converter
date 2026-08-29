use crate::api::Api;

mod config;
mod cache;
mod cli;
mod api;

const CONFIG_FILE: &'static str = "./Config.toml";

#[tokio::main]
async fn main() {
    let config = config::from(CONFIG_FILE);
    let args = cli::Args::new();
    let mut api = api::Api::new();

    let basecode = args.base.to_uppercase();

    let cache = cache::read_from_cache(&basecode);

    let rates;
    if let Some(r) = cache {
        // Cache hit
        rates = r;
    }
    else {
    // Cache miss
        api.get_key_from_file(&config.api.key_file);
        let response = api.request_rates(&basecode).await;
        match response {
            Ok(r) => rates = r,
            Err(e) => {Api::display_error(e); return;},
        }

        // Stringify and add to cache
        let string = serde_json::to_string(&rates).unwrap();
        cache::add_to_cache(&basecode, &string);
    }

    let targetcode = args.target.to_uppercase();
    if targetcode.is_empty() {
        println!("{}", serde_json::to_string_pretty(&rates.conversion_rates).unwrap());
        return;
    }

    let units = args.units;
    if rates.conversion_rates.contains_key(&targetcode)
    {
        let conversion_rate = rates.conversion_rates[&targetcode].as_f64().unwrap();
        let converted_units = conversion_rate * units;
        println!("{units} {basecode} = {converted_units} {targetcode}");
    }
}
