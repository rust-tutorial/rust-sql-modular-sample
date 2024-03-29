pub mod database;
pub mod status_code;
pub mod response;
pub mod logger;
pub mod ultil;
pub mod threadpool;
pub mod config;
pub mod error;
pub mod auth;
pub mod route;
pub mod request;
pub mod method;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
