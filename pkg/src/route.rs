use std::future::Future;
use std::net::TcpStream;

use crate::request::Request;

pub struct Routes<C, F> where
    C: Fn(Request) -> F,
    F: Future {
    routes: Vec<Box<Route<C, F>>>,
}

impl<C, F> Routes<C, F> where
    C: Fn(Request) -> F,
    F: Future {
    pub fn new(r: Vec<Box<Route<C, F>>>) -> Self {
        Routes { routes: r }
    }
    pub fn push(&mut self, r: Route<C, F>) {
        self.routes.push(Box::new(r));
    }
    pub fn search_route(&mut self, method: String, uri: String) -> Result<usize, usize> {
        let k = format!("{} {}", method, uri);
        self.routes.binary_search_by(|segment| segment.key.cmp(&k))
    }

    pub fn is_exist(&mut self, method: String, uri: String, index: usize) -> bool {
        let k = format!("{} {}", method, uri);
        match self.routes.binary_search_by(|r| r.key.cmp(&k)) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn add_route(&mut self, method: &str, uri: &str, handler_fn: C) {
        let r = Box::new(Route::new(method, uri, handler_fn));
        self.routes.push(r);
    }

    pub fn get_route(&mut self, i: usize) -> &Route<C, F> {
        &self.routes[i]
    }
}

pub struct Route<C, F> where
    C: Fn(Request) -> F,
    F: Future {
    pub key: String,
    pub handler_fn: C,
}

impl<C, F> Route<C, F> where
    C: Fn(Request) -> F,
    F: Future {
    pub fn new(method: &str, uri: &str, handler_fn: C) -> Self {
        Route { key: format!("{} {}", method, uri), handler_fn }
    }
}

