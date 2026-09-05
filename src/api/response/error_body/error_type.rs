#[derive(Debug)]
pub(crate) enum ErrorType {
    UnsupportedCode,
    MalformedRequest,
    InvalidKey,
    InactiveAccount,
    QuotaReached,
    Unknown(String),
}

impl From<String> for ErrorType {
    fn from(value: String) -> ErrorType {
        match value.as_str() {
            "unsupported-code" => ErrorType::UnsupportedCode,
            "malformed-request" => ErrorType::MalformedRequest,
            "invalid-key" => ErrorType::InvalidKey,
            "inactive-account" => ErrorType::InactiveAccount,
            "quota-reached" => ErrorType::QuotaReached,
            _ => ErrorType::Unknown(value),
        }
    }
}
