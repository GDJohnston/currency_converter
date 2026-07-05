use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ErrorResponse {
    pub(crate) result: String,
    #[serde(rename = "error-type")]
    pub(crate) error_type: String,
}
