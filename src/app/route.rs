use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::task::Context;
use std::boxed::Box;

use pkg::auth::{AuthError, from_authorization};
use pkg::logger::SimpleLogger;
use pkg::method::{DELETE, GET, PATCH, POST, PUT};
use pkg::request::{Request, RequestError};
use pkg::response::{json, Response, response};
use pkg::route::{Route, Routes};
use pkg::ultil::{get_param, load_query, parse_http_body, parse_route};
use serde_json::Value;
use tokio::net::UdpSocket;

use crate::configs::config::ApplicationConfig;
use crate::usecase::user::user::User;
use crate::usecase::user::user_handler::Handler;
use crate::usecase::user::user_repository::Repository;
use crate::usecase::user::user_service::Service;

pub async fn create_routes(mut stream: &TcpStream, h: &mut dyn Handler, cfg: ApplicationConfig) {
    let id_suffix = "{id}";
    let index = cfg.client.endpoint.index.clone();
    let s = stream.clone();
    let mut routes = Routes::new(vec![]);
    let route = Route::new(GET, format!("/users/{}", id_suffix).as_str(), Box::new(|req: Request| async {
        let mut l = dyn_clone::clone_box(&*h);
        l.load(s, req).await;
    }));
    routes.push(route);

    // let w1 = Route::new(POST, format!("/users/{}", id_suffix).as_str(), Box::new(|req: Request| async {
    //     let mut l = dyn_clone::clone_box(&*h);
    //     l.create(s, req).await;
    // }));
    // routes.push(w1);

    // routes.add_route(POST, format!("/users/{}", id_suffix).as_str(), |req: Request| async {
    //     let mut l = dyn_clone::clone_box(&*h);
    //     l.create(s, req).await;
    // });
    //
    // routes.add_route(PUT, format!("/users/{}", id_suffix).as_str(), |req: Request| async {
    //     let mut l = dyn_clone::clone_box(&*h);
    //     l.update(s, req).await;
    // });
    //
    // routes.add_route(PATCH, format!("/users/{}", id_suffix).as_str(), |req: Request| async {
    //     let mut l = dyn_clone::clone_box(&*h);
    //     l.patch(s, req).await;
    // });
    //
    // routes.add_route(DELETE, format!("/users/{}", id_suffix).as_str(), |req: Request| async {
    //     let mut l = dyn_clone::clone_box(&*h);
    //     l.delete(s, req).await;
    // });

    match Request::new_from_stream(stream) {
        Ok(request) => {
            match routes.search_route(request.method().to_string(), request.uri().to_string()) {
                Ok(i) => { (routes.get_route(i).handler_fn)(request).await }
                Err(_) => {
                    let index = index.clone();
                    let mut req_tokens: Vec<&str> = request.uri().split("/").collect();
                    let l = req_tokens.len();
                    req_tokens[l - 1 - index] = id_suffix;
                    let uri = req_tokens.join("/");
                    match routes.search_route(request.method().to_string(), uri) {
                        Ok(i) => { (routes.get_route(i).handler_fn)(request).await }
                        Err(_) => response(stream, "Not Found".to_string(), "application/json".to_string(), 404)
                    }
                }
            };
            // let m = request.method();
            // match m {
            //     pkg::method::GET => {
            //         let id = request.param(0);
            //         //h.load(stream, id.clone()).await;
            //         h.load(stream).await;
            //     }
            //     pkg::method::POST => {
            //         let user = serde_json::from_str::<User>(request.body()).unwrap();
            //         //h.create(stream, &user).await;
            //         h.create(stream).await;
            //     }
            //     pkg::method::PUT => {
            //         let user = serde_json::from_str::<User>(request.body()).unwrap();
            //         //h.update(stream, &user).await;
            //         h.update(stream).await;
            //     }
            //     pkg::method::PATCH => {
            //         let id = request.get_param("/users/{id}");
            //         let user = serde_json::from_str::<HashMap<String, Value>>(request.body()).unwrap();
            //         //h.patch(stream, id.clone(), user).await;
            //         h.patch(stream).await;
            //     }
            //     pkg::method::DELETE => {
            //         let id = request.get_param("/users/{id}");
            //         let user = serde_json::from_str::<HashMap<String, Value>>(request.body()).unwrap();
            //         //h.patch(stream, id.clone(), user).await;
            //         h.patch(stream).await;
            //     }
            //     _ => response(stream, "method not found".to_string(), "application/json".to_string(), 404)
            // }
        }
        Err(err) => response(stream, err.to_string(), "application/json".to_string(), 400)
    };
}
