use std::fmt;

#[derive(Debug)]
pub enum GoogleError {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    InvalidResponse(String),
    ApiError { message: String, code: Option<i32> },
}

impl fmt::Display for GoogleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GoogleError::Reqwest(e) => write!(f, "Request error: {}", e),
            GoogleError::SerdeJson(e) => write!(f, "JSON error: {}", e),
            GoogleError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            GoogleError::ApiError { message, code } => {
                write!(f, "API error: {} (code: {:?})", message, code)
            }
        }
    }
}

impl std::error::Error for GoogleError {}

impl From<reqwest::Error> for GoogleError {
    fn from(err: reqwest::Error) -> Self {
        GoogleError::Reqwest(err)
    }
}

impl From<serde_json::Error> for GoogleError {
    fn from(err: serde_json::Error) -> Self {
        GoogleError::SerdeJson(err)
    }
}
