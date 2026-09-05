use error_body::ErrorBody;
use rates_body::RatesBody;
use reqwest::Response;

pub mod error_body;
pub mod rates_body;

pub(crate) type Rates = rates_body::RatesBody;
pub(crate) type Error = error_body::ErrorBody;

pub(crate) async fn parse(response: Response) -> Result<Rates, Error> {
    let success = response.status().is_success();
    let body = response.text().await.unwrap();

    match success {
        true => Ok(RatesBody::new(&body)),
        false => Err(ErrorBody::new(&body)),
    }
}
