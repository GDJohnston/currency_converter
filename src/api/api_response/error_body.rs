mod error_type;
mod error_response;

#[derive(Debug)]
pub(crate) struct ErrorBody {
    result: String,
    error_type: error_type::ErrorType,
}

impl From<error_response::ErrorResponse> for ErrorBody {
    fn from(value: error_response::ErrorResponse) -> Self {
        ErrorBody{error_type: error_type::ErrorType::from(value.error_type), result: value.result}
    }
}

impl ErrorBody {
    pub(crate) fn new(response_body: &String) -> Self {
        dbg!(&response_body);
        let error_response: error_response::ErrorResponse = serde_json::from_str(&response_body).unwrap();
        ErrorBody::from(error_response)
    }
}
