use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ServiceError {
        http_code: u8,
        tracking_id: String,
        code: String,
        message: String,
    },

    HTTPClientError {
        description: String,
        cause: reqwest::Error,
    },

    WSHTTPClientError {
        description: String,
        cause: http::Error,
    },

    WSClientError {
        description: String,
        cause: tokio_tungstenite::tungstenite::Error,
    },

    SerializationError {
        description: String,
        cause: serde_json::error::Error,
    },

    GeneralError {
        description: String,
    },
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ServiceError {
                http_code: _,
                tracking_id: _,
                code: _,
                message: _,
            } => None,

            Error::HTTPClientError {
                description: _,
                cause: cause,
            } => Some(cause),

            Error::WSHTTPClientError {
                description: _,
                cause: cause,
            } => Some(cause),

            Error::WSClientError {
                description: _,
                cause: cause,
            } => Some(cause),

            Error::SerializationError {
                description: _,
                cause: cause,
            } => Some(cause),

            Error::GeneralError { description: _ } => None,
        }
    }

    fn description(&self) -> &str {
        match self {
            Error::ServiceError {
                http_code: _,
                tracking_id: _,
                code: _,
                message: message,
            } => message,

            Error::HTTPClientError {
                description: description,
                cause: _,
            } => description,

            Error::WSHTTPClientError {
                description: description,
                cause: _,
            } => description,

            Error::WSClientError {
                description: description,
                cause: _,
            } => description,

            Error::SerializationError {
                description: description,
                cause: _,
            } => description,

            Error::GeneralError {
                description: description,
            } => description,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ServiceError {
                http_code,
                tracking_id,
                code,
                message,
            } => write!(
                f,
                "ServiceError(http_code={}, tracking_id={}, code={}, message={})",
                http_code, tracking_id, code, message
            ),

            Error::HTTPClientError {
                description: description,
                cause: cause,
            } => write!(
                f,
                "HTTPClientError(description={}, cause={})",
                description, cause
            ),

            Error::WSHTTPClientError {
                description: description,
                cause: cause,
            } => write!(
                f,
                "WSHTTPClientError(description={}, cause={})",
                description, cause
            ),

            Error::WSClientError {
                description: description,
                cause: cause,
            } => write!(
                f,
                "WSClientError(description={}, cause={})",
                description, cause
            ),

            Error::SerializationError {
                description: description,
                cause: cause,
            } => write!(
                f,
                "SerializationError(description={}, cause={})",
                description, cause
            ),

            Error::GeneralError { description } => {
                write!(f, "GeneralError(description={})", description)
            }
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(serde_error: serde_json::error::Error) -> Self {
        Error::SerializationError {
            description: "Serde serialization/deserialization error".to_string(),
            cause: serde_error,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(reqwest_error: reqwest::Error) -> Self {
        Error::HTTPClientError {
            description: "HTTP request failed".to_string(),
            cause: reqwest_error,
        }
    }
}

impl From<http::Error> for Error {
    fn from(http_error: http::Error) -> Self {
        Error::WSHTTPClientError {
            description: "HTTP request failed".to_string(),
            cause: http_error,
        }
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(tungstenite_error: tokio_tungstenite::tungstenite::Error) -> Self {
        Error::WSClientError {
            description: "HTTP request failed".to_string(),
            cause: tungstenite_error,
        }
    }
}
