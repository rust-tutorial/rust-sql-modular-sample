use std::net::TcpStream;
use async_trait::async_trait;
use crate::usecase::user::user_service::Service;

#[async_trait]
pub trait Handler {
    async fn load(&mut self, mut stream: &TcpStream);
    async fn create(&mut self, mut stream: &TcpStream);
    async fn update(&mut self, mut stream: &TcpStream);
    async fn patch(&mut self, mut stream: &TcpStream);
    async fn delete(&mut self, mut stream: &TcpStream);
}

pub struct UserHandler {
    handler: Box<dyn Service + Send + Sync>,
}

impl UserHandler {
    pub fn new(handler: Box<dyn Service + Send + Sync>) -> Self {
        UserHandler { handler }
    }
}

#[async_trait]
impl Handler for UserHandler{
    async fn load(&mut self, mut stream: &TcpStream) {
        todo!()
    }

    async fn create(&mut self, mut stream: &TcpStream) {
        todo!()
    }

    async fn update(&mut self, mut stream: &TcpStream) {
        todo!()
    }

    async fn patch(&mut self, mut stream: &TcpStream) {
        todo!()
    }

    async fn delete(&mut self, mut stream: &TcpStream) {
        todo!()
    }
}
