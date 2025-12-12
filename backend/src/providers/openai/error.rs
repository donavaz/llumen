use std::fmt;

#[derive(Debug)]
pub enum OpenAIError {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    InvalidResponse(String),
    ApiError { message: String, code: Option<String> },
}

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenAIError::Reqwest(e) => write!(f, "Request error: {}", e),
            OpenAIError::SerdeJson(e) => write!(f, "JSON error: {}", e),
            OpenAIError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            OpenAIError::ApiError { message, code } => {
                write!(f, "API error: {} (code: {:?})", message, code)
            }
        }
    }
}

impl std::error::Error for OpenAIError {}

impl From<reqwest::Error> for OpenAIError {
    fn from(err: reqwest::Error) -> Self {
        OpenAIError::Reqwest(err)
    }
}

impl From<serde_json::Error> for OpenAIError {
    fn from(err: serde_json::Error) -> Self {
        OpenAIError::SerdeJson(err)
    }
}
