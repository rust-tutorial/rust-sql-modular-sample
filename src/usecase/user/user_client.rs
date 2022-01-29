use std::any::Any;
use std::collections::HashMap;
use std::task::Context;
use std::time::Duration;

use async_trait::async_trait;
use pkg::error::ApiError;
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::usecase::user::user::User;
use crate::usecase::user::user_repository::Repository;

#[derive(Debug, Clone)]
pub struct UserClient {
    client: reqwest::Client,
    url: String,
}

impl UserClient {
    pub fn new(timeout: u64, url: String) -> Self {
        UserClient {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(timeout))
                .build().expect("Error building new client")
            ,
            url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultInfo {
    status: i64,
    errors: Vec<ErrorMessage>,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorMessage {
    field: String,
    code: String,
    param: String,
    message: String,
}

#[async_trait]
impl Repository for UserClient {
    async fn load(&mut self, id: String) -> Result<User, ApiError> {
        let url = format!("{}/{}", self.url.clone(), id);
        let w = self.client.get(url).send().await;
        match w {
            Ok(resp) => {
                match resp.json::<User>().await {
                    Ok(u) => Ok(u),
                    Err(err) => Err(ApiError::Request(err))
                }
            }
            Err(err) => Err(ApiError::Request(err))
        }
    }

    async fn create(&mut self, user: &User) -> Result<i64, ApiError> {
        let url = self.url.clone();
        match serde_json::to_string::<User>(user) {
            Ok(body) => {
                match self.client.post(url).body(body).send().await {
                    Ok(res) => {
                        match res.json::<i64>().await {
                            Ok(info) => Ok(info),
                            Err(err) => Err(ApiError::Request(err))
                        }
                    }
                    Err(err) => Err(ApiError::Request(err))
                }
            }
            Err(err) => Err(ApiError::JsonError(err))
        }
    }

    async fn update(&mut self, user: &User) -> Result<i64, ApiError> {
        let url = format!("{}/{}", self.url.clone(), user.id.clone());
        match serde_json::to_string::<User>(user) {
            Ok(body) => {
                match self.client.put(url).body(body).send().await {
                    Ok(res) => {
                        match res.json::<i64>().await {
                            Ok(info) => Ok(info),
                            Err(err) => Err(ApiError::Request(err))
                        }
                    }
                    Err(err) => Err(ApiError::Request(err))
                }
            }
            Err(err) => Err(ApiError::JsonError(err))
        }
    }

    async fn patch(&mut self, id: String, user: HashMap<String, Value>) -> Result<i64, ApiError> {
        let url = format!("{}/{}", self.url.clone(), id.clone());
        match serde_json::to_string::<HashMap<String, Value>>(&user) {
            Ok(body) => {
                match self.client.patch(url).body(body).send().await {
                    Ok(res) => {
                        match res.json::<i64>().await {
                            Ok(info) => Ok(info),
                            Err(err) => Err(ApiError::Request(err))
                        }
                    }
                    Err(err) => Err(ApiError::Request(err))
                }
            }
            Err(err) => Err(ApiError::JsonError(err))
        }
    }

    async fn delete(&mut self, id: String) -> Result<i64, ApiError> {
        let url = format!("{}/{}", self.url.clone(), id);
        match self.client.delete(url).send().await {
            Ok(response) => {
                match response.json::<i64>().await {
                    Ok(info) => Ok(info),
                    Err(err) => Err(ApiError::Request(err))
                }
            }
            Err(err) => Err(ApiError::Request(err))
        }
    }
}
