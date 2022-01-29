use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::task::Context;

use pkg::response::json;
use pkg::ultil::{get_param, load_query, parse_http_body, parse_route};
use serde_json::Value;
use tokio::net::UdpSocket;

use crate::usecase::user::user::User;
use crate::usecase::user::user_handler::Handler;
use crate::usecase::user::user_repository::Repository;
use crate::configs::config::ApplicationConfig;
use pkg::auth::{auth_middleware, AuthError};

pub async fn create_routes(mut stream: &TcpStream, h: &mut dyn Handler, cfg: ApplicationConfig) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read stream");
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let mut request_content = String::from_utf8_lossy(&buffer[..]).to_string();
    request_content = request_content.trim_end_matches(char::from(0)).to_string();

    let url_content = request_content.clone();
    let url: Vec<&str> = url_content.split(" ").collect();

    let id = get_param(url[1].to_string().clone().clone());
    let login = parse_route("POST".to_string(), "/auth/login".to_string());
    let create_credential = parse_route("POST".to_string(), "/auth/create".to_string());
    let create = parse_route("POST".to_string(), "/users".to_string());
    let load = parse_route("GET".to_string(), format!("/users/{}", id));
    let update = parse_route("PUT".to_string(), format!("/users/{}", id));
    let patch = parse_route("PATCH".to_string(), format!("/users/{}", id));
    match buffer {
        b if b.starts_with(create.as_bytes()) => {
            match parse_http_body(request_content) {
                Ok(val) => {
                    let user = serde_json::from_str::<User>(val.as_str()).unwrap();
                    h.create(stream, &user).await;
                }
                Err(err) => json(stream, err.to_string(), "HTTP/1.1 400 Bad Request".to_string()),
            }
        }
        b if b.starts_with(load.as_bytes()) => {
            let content = request_content.clone();
            match auth_middleware(cfg.server.secret_key, content) {
                Ok(()) => h.load(stream, id.clone()).await,
                Err(err) => json(stream, err.to_string(), "HTTP/1.1 401 Unauthorized".to_string())
            };
        }
        b if b.starts_with(b"DELETE") => {
            h.delete(stream, id.clone()).await;
        }
        b if b.starts_with(update.as_bytes()) => {
            match parse_http_body(request_content) {
                Ok(val) => {
                    let user = serde_json::from_str::<User>(val.as_str()).unwrap();
                    h.update(stream, &user).await;
                }
                Err(err) => json(stream, err.to_string(), "HTTP/1.1 400 OK".to_string()),
            }
        }
        b if b.starts_with(patch.as_bytes()) => {
            match parse_http_body(request_content) {
                Ok(val) => {
                    let user = serde_json::from_str::<HashMap<String, Value>>(val.as_str()).unwrap();
                    h.patch(stream, id.clone(), user).await;
                }
                Err(err) => json(stream, err.to_string(), "HTTP/1.1 400 OK".to_string()),
            }
        }
        _ => json(stream, "Not Found".to_string(), "HTTP/1.1 404".to_string()),
    };
}
