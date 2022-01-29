use std::future::Future;
use std::net::TcpStream;

pub struct Route<C, F> where
    C: Fn(&TcpStream) -> F,
    F: Future {
    pub method: String,
    pub uri: String,
    pub handler_fn: C,
}

impl<C, F> Route<C, F>
    where F: Future,
          C: Fn(&TcpStream) -> F,
{
    pub async fn register(self, stream: &TcpStream) {
        (self.handler_fn)(stream).await;
    }
}

