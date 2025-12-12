use std::fmt;

#[derive(Debug)]
pub enum AnthropicError {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    InvalidResponse(String),
    ApiError { message: String, error_type: Option<String> },
}

impl fmt::Display for AnthropicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnthropicError::Reqwest(e) => write!(f, "Request error: {}", e),
            AnthropicError::SerdeJson(e) => write!(f, "JSON error: {}", e),
            AnthropicError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            AnthropicError::ApiError { message, error_type } => {
                write!(f, "API error: {} (type: {:?})", message, error_type)
            }
        }
    }
}

impl std::error::Error for AnthropicError {}

impl From<reqwest::Error> for AnthropicError {
    fn from(err: reqwest::Error) -> Self {
        AnthropicError::Reqwest(err)
    }
}

impl From<serde_json::Error> for AnthropicError {
    fn from(err: serde_json::Error) -> Self {
        AnthropicError::SerdeJson(err)
    }
}
