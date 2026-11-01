//! Error response from API
use error_response::ErrorResponse;
use error_type::ErrorType;

mod error_response;
mod error_type;

/// Parsed error response from API
#[derive(Debug)]
pub(crate) struct ErrorBody {
    error_type: ErrorType,
}

impl From<ErrorResponse> for ErrorBody {
    /// Parse error error type from the parse error response
    fn from(value: ErrorResponse) -> Self {
        ErrorBody {
            error_type: ErrorType::from(value.error_type),
        }
    }
}

impl ErrorBody {
    /// Create a new ErrorBody type from the API response
    pub(crate) fn new(response_body: &String) -> Self {
        let error_response: ErrorResponse = response_body.into();
        ErrorBody::from(error_response)
    }

    /// Display the parsed error type
    pub(crate) fn display(self) {
        if let ErrorType::Unknown(error) = self.error_type {
            eprintln!("Unknown error type: {:#?}", error);
        } else {
            eprintln!("{:#?}", self.error_type);
        }
    }
}
