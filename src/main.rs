use crate::api_response::ApiResponse::{Error, Rates};

mod api_request;
mod api_response;
mod config;
mod cache;
mod cli;

const CONFIG_FILE: &'static str = "./Config.toml";

#[tokio::main]
async fn main() {
    let config = config::get_config(CONFIG_FILE);
    let args = cli::Args::new();

    let api_key = config.api.key;
    let basecode = args.base.to_uppercase();

    let cache = cache::read_from_cache(&basecode);
    let rates_body =
    if let Some(rates_body) = cache {
        rates_body
    }
    else {
        let response = api_request::request(&api_key, &basecode).await;
        let rates_body = match response {
            Rates(rates_body) => rates_body,
            Error(error_body) => {println!("{:#?}", error_body); return;},
        };
        let string = serde_json::to_string(&rates_body).unwrap();
        cache::add_to_cache(&basecode, &string);
        rates_body
    };

    let targetcode = args.target.to_uppercase();
    if targetcode.is_empty() {
        println!("{:#?}", rates_body.conversion_rates);
        return;
    }

    let units = args.units;
    if rates_body.conversion_rates.contains_key(&targetcode)
    {
        let conversion_rate = rates_body.conversion_rates[&targetcode].as_f64().unwrap();
        let converted_units = conversion_rate * units;
        println!("{units} {basecode} = {converted_units} {targetcode}");
    }
}
