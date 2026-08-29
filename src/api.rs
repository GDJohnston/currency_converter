use std::path::PathBuf;

pub(crate) mod api_key;
pub(crate) mod api_response;

const WEBSITE: &'static str = "https://v6.exchangerate-api.com/v6/";

pub(crate) struct Api {
    key: Option<String>,
}

impl Api {
    pub(crate) fn new() -> Self {
        Api { key: None }
    }

    pub(crate) fn get_key_from_file(&mut self, key_file: &PathBuf) -> &Self {
        let key = api_key::from_file(key_file.as_path());
        self.key = Some(key);
        self
    }

    pub(crate) async fn request_rates(&self, basecode: &str) -> Result<api_response::Rates, api_response::Error> {
        let key = self
            .key
            .as_ref()
            .expect("Key must be set before requesting rates");

        let url: PathBuf = [WEBSITE, key.as_str(), "latest/", basecode].iter().collect();
        let url = url.to_str().unwrap();
        dbg!(&url);
        let response = reqwest::get(url).await.unwrap();
        dbg!(&response);

        api_response::parse(response).await
    }

    pub(crate) fn display_error(error: api_response::Error) {
        println!("{:#?}", error)
    }
}
