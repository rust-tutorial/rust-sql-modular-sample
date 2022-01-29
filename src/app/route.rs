use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::task::Context;

use pkg::auth::{AuthError, from_authorization};
use pkg::logger::SimpleLogger;
use pkg::request::{Request, RequestError};
use pkg::response::{json, Response, response};
//use pkg::route::Route;
use pkg::ultil::{get_param, load_query, parse_http_body, parse_route};
use serde_json::Value;
use tokio::net::UdpSocket;

use crate::configs::config::ApplicationConfig;
use crate::usecase::user::user::User;
use crate::usecase::user::user_repository::Repository;
use crate::usecase::user::user_service::Service;
use crate::usecase::user::user_handler::Handler;
use pkg::route::Route;

pub async fn create_routes(mut stream: &TcpStream, h: &mut dyn Handler, cfg: ApplicationConfig) {
    // let s = S { foo };
    // s.do_thing().await;
    // let route = Route{
    //     method: "".to_string(),
    //     uri: "".to_string(),
    //     handler_fn: h.load(stream),
    // };
    // route.handler_fn().await;
    match Request::new_from_stream(stream) {
        Ok(req) => {
            let m = req.method();
            match m {
                pkg::method::GET => {
                    let id = req.get_param("/users/{id}");
                    //h.load(stream, id.clone()).await;
                    h.load(stream).await;
                }
                pkg::method::POST => {
                    let user = serde_json::from_str::<User>(req.body()).unwrap();
                    //h.create(stream, &user).await;
                    h.create(stream).await;
                }
                pkg::method::PUT => {
                    let user = serde_json::from_str::<User>(req.body()).unwrap();
                    //h.update(stream, &user).await;
                    h.update(stream).await;
                }
                pkg::method::PATCH => {
                    let id = req.get_param("/users/{id}");
                    let user = serde_json::from_str::<HashMap<String, Value>>(req.body()).unwrap();
                    //h.patch(stream, id.clone(), user).await;
                    h.patch(stream).await;
                }
                pkg::method::DELETE => {
                    let id = req.get_param("/users/{id}");
                    let user = serde_json::from_str::<HashMap<String, Value>>(req.body()).unwrap();
                    //h.patch(stream, id.clone(), user).await;
                    h.patch(stream).await;
                }
                _ => response(stream, "method not found".to_string(), "application/json".to_string(), 404)
            }
        }
        Err(err) => response(stream, err.to_string(), "application/json".to_string(), 400)
    };
}
