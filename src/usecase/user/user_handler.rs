use std::collections::HashMap;
use std::net::TcpStream;

use async_trait::async_trait;
use dyn_clone::DynClone;
use pkg::request::Request;
use serde_json::Value;

use crate::usecase::user::user::User;
use crate::usecase::user::user_service::Service;

#[async_trait]
pub trait Handler: DynClone {
    async fn load(&mut self, mut stream: &TcpStream, request: Request);
    async fn create(&mut self, mut stream: &TcpStream, request: Request);
    async fn update(&mut self, mut stream: &TcpStream, request: Request);
    async fn patch(&mut self, mut stream: &TcpStream, request: Request);
    async fn delete(&mut self, mut stream: &TcpStream, request: Request);
}

dyn_clone::clone_trait_object!(Handler);

#[derive(Clone)]
pub struct UserHandler {
    handler: Box<dyn Service + Send + Sync>,
}

impl UserHandler {
    pub fn new(handler: Box<dyn Service + Send + Sync>) -> Self {
        UserHandler { handler }
    }
}

#[async_trait]
impl Handler for UserHandler {
    async fn load(&mut self, mut stream: &TcpStream, request: Request) {
        let id = request.param(0);
        self.handler.load(stream, id.to_string());
    }

    async fn create(&mut self, mut stream: &TcpStream, request: Request) {
        let user = serde_json::from_str::<User>(request.body()).unwrap();
        self.handler.create(stream, &user);
    }

    async fn update(&mut self, mut stream: &TcpStream, request: Request) {
        let user = serde_json::from_str::<User>(request.body()).unwrap();
        self.handler.update(stream, &user);
    }

    async fn patch(&mut self, mut stream: &TcpStream, request: Request) {
        let id = request.param(0);
        let user = serde_json::from_str::<HashMap<String, Value>>(request.body()).unwrap();
        self.handler.patch(stream, id.to_string(), user);
    }

    async fn delete(&mut self, mut stream: &TcpStream, request: Request) {
        let id = request.param(0);
        self.handler.delete(stream, id.to_string());
    }
}
