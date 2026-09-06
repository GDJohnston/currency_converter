//! Error response from API
use error_type::ErrorType;
use error_response::ErrorResponse;

mod error_type;
mod error_response;

/// Parsed error response from API
#[derive(Debug)]
pub(crate) struct ErrorBody {
    result: String,
    error_type: ErrorType,
}

impl From<ErrorResponse> for ErrorBody {
    /// Parse error error type from the parse error response
    fn from(value: ErrorResponse) -> Self {
        ErrorBody{error_type: ErrorType::from(value.error_type), result: value.result}
    }
}

impl ErrorBody {
    /// Create a new ErrorBody type from the API response
    pub(crate) fn new(response_body: &String) -> Self {
        let error_response: ErrorResponse = response_body.into();
        ErrorBody::from(error_response)
    }
}
