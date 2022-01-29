use serde_json;

use reqwest;

pub enum ApiError {
    Request(reqwest::Error),
    JsonError(serde_json::Error),
    Database(mysql::Error),
    NotFound
}
