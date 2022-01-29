use std::fmt;
use std::fmt::Display;
use std::io::Write;
use std::net::TcpStream;

use log::{error, info};

const BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
const NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
const SUCCESS: &str = "HTTP/1.1 200 OK";
const UNAUTHORIZED: &str = "HTTP/1.1 401 Unauthorized";
const FORBIDDEN: &str = "HTTP/1.1 403 Forbidden";
const INTERNAL: &str = "HTTP/1.1 500 Internal Server Error";

pub enum ResponseError {
    WriteErr(std::io::Error),
    FlushErr(std::io::Error),
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResponseError::WriteErr(err) => write!(f, "{}", err.to_string()),
            ResponseError::FlushErr(err) => write!(f, "{}", err.to_string()),
        }
    }
}

pub struct Response {
    content: String,
    content_type: String,
}

impl Response {
    pub fn new(content: String, content_type: String) -> Self {
        Response { content, content_type }
    }
    pub fn json(&mut self, mut stream: &TcpStream, code: u64) -> Result<(), ResponseError> {
        let mut status_line: String = Default::default();
        match code {
            200 => status_line = SUCCESS.to_string(),
            400 => status_line = BAD_REQUEST.to_string(),
            401 => status_line = UNAUTHORIZED.to_string(),
            403 => status_line = FORBIDDEN.to_string(),
            500 => status_line = INTERNAL.to_string(),
            _ => status_line = NOT_FOUND.to_string(),
        }
        let response = format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            self.content_type.clone(),
            self.content.clone().len(),
            self.content.clone(),
        );
        match stream.write(response.as_bytes()) {
            Ok(_) =>
                return match stream.flush() {
                    Ok(_) => Ok(()),
                    Err(err) => Err(ResponseError::FlushErr(err))
                },
            Err(err) => Err(ResponseError::WriteErr(err))
        }
    }
}

pub fn response(mut stream: &TcpStream, content: String, content_type: String, code: u64) {
    let mut r = Response::new(
        content,
        content_type,
    );
    match r.json(stream, code) {
        Ok(_) => info!("response sent"),
        Err(err) => error!("{}", err)
    };
}


pub fn json(mut stream: &TcpStream, contents: String, status_line: String) {
    let response = format!(
        "{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).expect("Failed to write");
    stream.flush().expect("Failed to flush");
}
