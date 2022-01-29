use std::fmt::Display;
use std::str::FromStr;
use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;

use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub phone: String,
    #[serde(rename(serialize = "dateOfBirth", deserialize = "dateOfBirth"))]
    pub date_of_birth: String,
}

impl User {
    pub fn new(id: String, username: String, email: String, phone: String, date_of_birth: String) -> Self {
        User { id, username, email, phone, date_of_birth }
    }
}

