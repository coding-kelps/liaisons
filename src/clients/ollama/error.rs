use std::fmt;

pub enum Error {
    RequestError(reqwest::Error),
    
    ApiError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RequestError(e) => write!(f, "request error - {}", e),
            Error::ApiError(e) => write!(f, "api error - {}", e),
        }
    }
}
