//! Parsed error response
use serde::{Deserialize, Serialize};

/// Serialised API errror response
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ErrorResponse {
    pub(crate) result: String,
    #[serde(rename = "error-type")]
    pub(crate) error_type: String,
}

impl From<&String> for ErrorResponse {
    /// Serialise [`ErrorResponse`] from an API error response
    fn from(response_body: &String) -> Self {
        serde_json::from_str(response_body).unwrap()
    }
}
