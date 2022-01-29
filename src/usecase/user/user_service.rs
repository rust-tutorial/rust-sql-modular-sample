use std::any::Any;
use std::collections::HashMap;
use std::convert::Infallible;
use std::io::Write;
use std::net::TcpStream;
use std::str::FromStr;
use std::task::Context;

use async_trait::async_trait;
use bson::Document;
use bson::extjson::de::Error::InvalidObjectId;
use bson::oid::ObjectId;
use futures::TryStreamExt;
use log::{info, trace, warn};
use mongodb::Cursor;
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult};
use pkg::error::ApiError;
use pkg::response::{json, Response};
use pkg::status_code::{BAD_REQUEST, INTERNAL_SERVER_ERROR, SUCCESS};
use pkg::ultil::{parse_http_body, ParseBodyError};
use serde_json::Value;

use crate::usecase::user::user::User;
use crate::usecase::user::user_repository::Repository;
use dyn_clone::DynClone;

#[async_trait]
pub trait Service: DynClone {
    async fn load(&mut self, mut stream: &TcpStream, id: String);
    async fn create(&mut self, mut stream: &TcpStream, user: &User);
    async fn update(&mut self, mut stream: &TcpStream, user: &User);
    async fn patch(&mut self, mut stream: &TcpStream, id: String, user: HashMap<String, Value>);
    async fn delete(&mut self, mut stream: &TcpStream, id: String);
}

dyn_clone::clone_trait_object!(Service);

fn handle_response(mut stream: &TcpStream, res: Result<i64, ApiError>) {
    match res {
        Ok(v) => {
            match v {
                -1 => json(stream, (-1).to_string(), BAD_REQUEST.to_string()),
                value => json(stream, value.to_string(), SUCCESS.to_string())
            }
        }
        Err(err) => handle_error(stream, err)
    }
}

fn handle_error(mut stream: &TcpStream, err: ApiError) {
    match err {
        ApiError::Request(err) => json(stream, err.to_string(), INTERNAL_SERVER_ERROR.to_string()),
        ApiError::JsonError(err) => json(stream, err.to_string(), BAD_REQUEST.to_string()),
        _ => json(stream, "unavailable server".to_string(), INTERNAL_SERVER_ERROR.to_string())
    }
}

#[derive(Clone)]
pub struct UserService {
    service: Box<dyn Repository + Send + Sync>,
}

impl UserService {
    pub fn new(service: Box<dyn Repository + Send + Sync>) -> Self {
        UserService { service }
    }
}

#[async_trait] // Currently async trait is not supported but the restriction will be removed in the future
impl Service for UserService {
    async fn load(&mut self, mut stream: &TcpStream, id: String) {
        let res = self.service.load(id).await;
        match res {
            Ok(v) => {
                match serde_json::to_string::<User>(&v) {
                    Ok(r) => json(stream, r, SUCCESS.to_string()),
                    Err(err) => json(stream, err.to_string(), INTERNAL_SERVER_ERROR.to_string())
                }
            }
            Err(e) => {
                let mut r = Response::new(e.to_string(), "application/json".to_string());
                r.json(stream, 500);
            }, //handle_error(stream, e)
        }
    }

    async fn create(&mut self, mut stream: &TcpStream, user: &User) {
        let res = self.service.create(user).await;
        handle_response(stream, res);
    }

    async fn update(&mut self, mut stream: &TcpStream, user: &User) {
        let res = self.service.update(user).await;
        handle_response(stream, res);
    }

    async fn patch(&mut self, mut stream: &TcpStream, id: String, user: HashMap<String, Value>) {
        let res = self.service.patch(id, user).await;
        handle_response(stream, res);
    }

    async fn delete(&mut self, mut stream: &TcpStream, id: String) {
        let res = self.service.delete(id).await;
        handle_response(stream, res);
    }
}
