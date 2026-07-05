use crate::api_response;

const WEBSITE: &'static str = "https://v6.exchangerate-api.com/v6/";

pub(crate) async fn request(api_key: &str, basecode: &str) -> api_response::ApiResponse {
    let response = reqwest::get(WEBSITE.to_owned() + api_key + "/latest" + "/" + basecode)
        .await.unwrap();

    println!("{:#?}", response);
    let response = api_response::ApiResponse::new(response).await;
    println!("requested from api");
    response
}