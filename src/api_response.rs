pub mod rates_body;
mod error_body;

#[derive(Debug)]
pub(crate) enum ApiResponse {
    Rates(rates_body::RatesBody),
    Error(error_body::ErrorBody),
}

impl ApiResponse
{
    pub(crate) async fn new(response: reqwest::Response) -> Self {
        let success = response.status().is_success(); 
        let body = response.text().await.unwrap();

        match success {
            true => ApiResponse::Rates(rates_body::RatesBody::new(&body)),
            false => ApiResponse::Error(error_body::ErrorBody::new(&body)),
        }
    }
}
