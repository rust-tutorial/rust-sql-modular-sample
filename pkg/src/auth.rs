use std::fmt::Display;

use argon2::{Argon2, password_hash::{
    Error, PasswordHash,
    PasswordHasher, PasswordVerifier, rand_core::OsRng, SaltString,
}};
use chrono::{Duration, Utc};
use cookie::Cookie;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub enum AuthError {
    TokenNotFound,
    DecodeError(jsonwebtoken::errors::Error),
    ExpiredJwt,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::TokenNotFound => write!(f, "missing token"),
            AuthError::ExpiredJwt => write!(f, "token is expired"),
            AuthError::DecodeError(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Credential {
    username: String,
    password: String,
    #[serde(skip_serializing)]
    hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: i64,
}

pub fn hashing_password(password: String) -> Result<String, Error> {
    let password = password.as_bytes(); // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    match argon2.hash_password(password, &salt) {
        Ok(phc) => Ok(phc.to_string()),
        Err(err) => Err(err)
    }
}

pub fn compare_hash(password: String, hash_password: String) -> Result<(), Error> {
    match PasswordHash::new(&hash_password) {
        Ok(parsed_hash) => {
            let password_bytes = password.as_bytes();
            Argon2::default().verify_password(password_bytes, &parsed_hash)
        }
        Err(err) => Err(err)
    }
}

pub fn jwt_signing(secret_key: String, duration: i64) -> String {
    let expire = chrono::Utc::now() + Duration::seconds(duration);
    let my_claims = Claims {
        sub: "tma.com.vn".to_owned(),
        company: "TMA".to_owned(),
        exp: expire.timestamp(),
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secret_key.as_ref())).unwrap();
    token
}

pub fn jwt_verify(secret_key: String, token: String) -> Result<bool, jsonwebtoken::errors::Error> {
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    ) {
        Ok(tokenData) => {
            if tokenData.claims.exp < chrono::Utc::now().timestamp() {
                return Ok(false);
            } else {
                return Ok(true);
            }
        }
        Err(err) => Err(err)
    }
}

pub fn from_authorization(secret_key: String, request_content: String) -> Result<(), AuthError> {
    let content = request_content.clone();
    let parts: Vec<&str> = content.split("\r\n").collect();
    for p in parts {
        if p.contains("Authorization") {
            let p = p.to_string();
            let tokens: Vec<&str> = p.split(" ").collect();
            return match tokens.last() {
                None => Err(AuthError::TokenNotFound),
                Some(t) =>
                    match jwt_verify(secret_key, t.to_string()) {
                        Ok(b) =>
                            match b {
                                true => Ok(()),
                                false => Err(AuthError::ExpiredJwt)
                            }
                        Err(err) => Err(AuthError::DecodeError(err))
                    }
            };
        }
    };
    Err(AuthError::TokenNotFound)
}

pub fn from_cookie(secret_key: String, request_content: String)-> Result<(), AuthError> {
    let content = request_content.clone();
    let parts: Vec<&str> = content.split("\r\n").collect();
    for p in parts {
        if p.contains("Cookie") {
            let p = p.to_string();
            let tokens: Vec<&str> = p.split(";").collect();
            for t in tokens {
                if t.contains("token") {
                    let tokens: Vec<&str> = t.split("=").collect();
                    return match tokens.last() {
                        None => Err(AuthError::TokenNotFound),
                        Some(t) =>
                            match jwt_verify(secret_key, t.to_string()) {
                                Ok(b) =>
                                    match b {
                                        true => Ok(()),
                                        false => Err(AuthError::ExpiredJwt)
                                    }
                                Err(err) => Err(AuthError::DecodeError(err))
                            }
                    };
                }
            };
        }
    };
    Err(AuthError::TokenNotFound)
}

pub fn create_cookie(key: String, token: String, domain: String, path: String, secure: bool, http_only: bool) -> String {
    let cookie = Cookie::build(key, token)
        .domain(domain.as_str())
        .path(path.as_str())
        .secure(secure)
        .http_only(http_only)
        .finish();
    cookie.to_string()
}
