use serde_json;

use reqwest;
use std::fmt;
use std::fmt::Display;

pub enum ApiError {
    Request(reqwest::Error),
    JsonError(serde_json::Error),
    Database(mysql::Error),
    NotFound
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Request(err) => write!(f, "{}", err.to_string()),
            ApiError::JsonError(err) => write!(f, "{}", err.to_string()),
            ApiError::Database(err) => write!(f, "{}", err.to_string()),
            ApiError::NotFound => write!(f, "not found"),
        }
    }
}

// pub fn handle_error(mut stream: &TcpStream, err: ApiError) {
//     match err {
//         ApiError::Request(err) => json(stream, err.to_string(), INTERNAL_SERVER_ERROR.to_string()),
//         ApiError::JsonError(err) => json(stream, err.to_string(), BAD_REQUEST.to_string()),
//         _ => json(stream, "unavailable server".to_string(), INTERNAL_SERVER_ERROR.to_string())
//     }
// }
