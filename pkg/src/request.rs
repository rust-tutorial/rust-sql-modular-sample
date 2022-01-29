use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::Read;
use std::net::TcpStream;

use crate::method::is_method;

pub enum RequestError {
    ReadStream(std::io::Error),
    MalformRequest,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::ReadStream(err) => write!(f, "{}", err.to_string()),
            RequestError::MalformRequest => write!(f, "invalid request"),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    method: String,
    uri: String,
    protocol: String,
    headers: HashMap<String, String>,
    body: String,
}

fn parse_body(raw_body: String) -> (String, String) {
    let v: Vec<&str> = raw_body.split("\r\n\r\n").collect();
    match v.last() {
        None => (v.first().unwrap().to_string(), "".to_string()),
        Some(body) => (v.first().unwrap().to_string(), body.to_string()),
    }
}

impl Request {
    pub fn new(method: String, uri: String, protocol: String, headers: HashMap<String, String>, body: String) -> Self {
        Request { method, uri, protocol, headers, body }
    }
    pub fn method(&self) -> &str {
        &self.method
    }
    pub fn uri(&self) -> &str {
        &self.uri
    }
    pub fn protocol(&self) -> &str {
        &self.protocol
    }
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    pub fn body(&self) -> &str {
        &self.body
    }
    pub fn new_from_stream(mut stream: &TcpStream) -> Result<Self, RequestError> {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {
                let mut headers = HashMap::<String, String>::new();
                let mut method: String = Default::default();
                let mut uri: String = Default::default();
                let mut protocol: String = Default::default();
                let mut body: String = Default::default();
                let mut request_content = String::from_utf8_lossy(&buffer[..]).to_string();
                request_content = request_content.trim_end_matches(char::from(0)).to_string();
                let t = parse_body(request_content.clone());
                request_content = t.0;
                body = t.1;
                let req_cont = request_content.clone();
                let contents: Vec<&str> = req_cont.split("\r\n").collect();
                for r in contents {
                    if is_method(r.to_string()) {
                        let m: Vec<&str> = r.split(" ").collect();
                        if m.len() != 3 {
                            return Err(RequestError::MalformRequest);
                        } else {
                            method = m[0].to_string();
                            uri = m[1].to_string();
                            protocol = m[2].to_string();
                        }
                    } else {
                        if !r.contains("{") && !r.contains("}") {
                            let header: Vec<&str> = r.split(": ").collect();
                            headers.insert(header.first().unwrap().to_string(), header.last().unwrap().to_string());
                        } else {
                            body = r.to_string().trim_start_matches(char::from(0)).to_string();
                        }
                    };
                }
                Ok(Request {
                    method,
                    uri,
                    protocol,
                    headers,
                    body,
                })
            }
            Err(err) => Err(RequestError::ReadStream(err))
        }
    }
    pub fn get_param(&self, pattern: &str) -> String {
        let uri = self.uri.clone();
        let u: Vec<&str> = uri.split("/").collect();
        let p: Vec<&str> = pattern.split("/").collect();
        if u.len() != p.len() {
            return "".to_string();
        } else {
            for (i, v) in u.iter().enumerate() {
                if u[i] != p[i] {
                    return v.to_string();
                }
            };
            return "".to_string();
        }
    }
}
