use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use pkg::auth::{compare_hash, jwt_signing, create_cookie};
use pkg::threadpool::ThreadPool;
use pkg::ultil::{parse_http_body, parse_route};

use crate::app::route::create_routes;
use crate::configs::config;
use crate::usecase::user::user::User;
use crate::usecase::user::user_client::UserClient;
use crate::usecase::user::user_service::UserService;
use crate::usecase::user::user_repository::UserRepository;
use crate::usecase::user::user_handler::UserHandler;

mod app;
mod configs;
mod usecase;

#[tokio::main]
async fn main() {
    let token = jwt_signing("gPHpdHAplEE6qLPE".to_string(), 120);
    println!("{}", token);
    let ck = create_cookie("token".to_string(), token, "localhost".to_string(), "/".to_string(), true, true);
    println!("{}", ck);


    pkg::logger::init().expect("Error init log");
    let cfg = config::ApplicationConfig::load_yaml_config("./config/Settings.yaml".to_string());
    let pool = pkg::database::connect(cfg.server.url.clone());
    let listener = TcpListener::bind(format!("127.0.0.1:{}", cfg.server.port.clone()))
        .expect("Failed to bind address");
    let threads = ThreadPool::new(cfg.max_threads);
    println!("HTTP server started at {}", cfg.server.port.clone());
    for stream in listener.incoming() {
        let stream = stream.expect("Connection failed");
        let endpoint = cfg.client.endpoint.clone();
        threads.execute({
            let pool = pool.clone();
            let cfg = cfg.clone();
            move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let user_client = UserClient::new(endpoint.timeout.clone(), endpoint.url.clone());
                let user_repository = UserRepository::new("users".to_string(), pool);
                let mut user_service = UserService::new(Box::new(user_client));
                let mut user_handler = UserHandler::new(Box::new(user_service));
                rt.block_on(create_routes(&stream, &mut user_handler, cfg));
            }
        });
    }
    println!("Shutting down.");
}
