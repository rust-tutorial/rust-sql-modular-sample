use std::any::Any;
use std::collections::HashMap;
use std::fmt::Display;
use std::task::Context;

use async_trait::async_trait;
use mongodb::bson::Document;
use mongodb::Collection;
use mysql::{Error, PooledConn, Row, Transaction, TxOpts, Params};
use mysql::params;
use mysql::prelude::{Queryable, WithParams};
use pkg::error::ApiError;
use serde_json::Value;
use tokio::runtime::Runtime;

use crate::app::query::{query_create, query_delete, query_load, query_patch, query_update};
use crate::usecase::user::user::User;

#[async_trait]
pub trait Repository {
    async fn load(&mut self, id: String) -> Result<User, ApiError>;
    async fn create(&mut self, user: &User) -> Result<i64, ApiError>;
    async fn update(&mut self, user: &User) -> Result<i64, ApiError>;
    async fn patch(&mut self, id: String, user: HashMap<String, Value>) -> Result<i64, ApiError>;
    async fn delete(&mut self, id: String) -> Result<i64, ApiError>;
}

pub struct UserRepository {
    table: String,
    conn: PooledConn,
}

impl UserRepository {
    pub fn new(table: String, c: PooledConn) -> Self {
        UserRepository {
            table,
            conn: c,
        }
    }
}


#[async_trait]
impl Repository for UserRepository {
    async fn load(&mut self, id: String) -> Result<User, ApiError> {
        let query = query_load::<String>(self.table.clone(), id);
        let mut tx = self.conn.start_transaction(TxOpts::default()).unwrap();
        let v = tx.query_first(query).map(|user| {
            user.map(|(id, username, email, phone, date_of_birth)| User {
                id,
                username,
                email,
                phone,
                date_of_birth,
            })
        });
        match v {
            Ok(v) => {
                match v {
                    None => {
                        tx.rollback();
                        Err(ApiError::NotFound)
                    }
                    Some(u) => {
                        tx.commit();
                        Ok(u)
                    }
                }
            }
            Err(err) => {
                tx.rollback();
                Err(ApiError::Database(err))
            }
        }
    }

    async fn create(&mut self, user: &User) -> Result<i64, ApiError> {
        let query = query_create(self.table.clone());
        let mut tx = self.conn.start_transaction(TxOpts::default()).unwrap();
        let result = tx.exec_drop(query, (
            user.id.clone(),
            user.username.clone(),
            user.email.clone(),
            user.phone.clone(),
            user.date_of_birth.clone()));
        match result {
            Ok(_) => {
                tx.commit();
                Ok(1)
            }
            Err(err) => {
                tx.rollback();
                Err(ApiError::Database(err))
            }
        }
    }

    async fn update(&mut self, user: &User) -> Result<i64, ApiError> {
        let query = query_update(self.table.clone());
        let mut tx = self.conn.start_transaction(TxOpts::default()).unwrap();
        let update_result = tx.exec_drop(query, (
            user.username.clone(),
            user.email.clone(),
            user.phone.clone(),
            user.date_of_birth.clone(), user.id.clone()));
        match update_result {
            Ok(_) => {
                tx.commit();
                Ok(1)
            }
            Err(err) => {
                tx.rollback();
                Err(ApiError::Database(err))
            }
        }
    }

    async fn patch(&mut self, id: String, user: HashMap<String, Value>) -> Result<i64, ApiError> {
        let query = query_patch(self.table.clone(), id, user);
        let mut tx = self.conn.start_transaction(TxOpts::default()).unwrap();
        let patch_result = tx.exec_drop(query, Params::Empty);
        match patch_result {
            Ok(_) => {
                tx.commit();
                Ok(1)
            }
            Err(err) => {
                tx.rollback();
                Err(ApiError::Database(err))
            }
        }
    }

    async fn delete(&mut self, id: String) -> Result<i64, ApiError> {
        let query = query_delete(self.table.clone());
        let mut tx = self.conn.start_transaction(TxOpts::default()).unwrap();
        let delete_result = tx.exec_drop(query, params! {id});
        match delete_result {
            Ok(_) => {
                tx.commit();
                Ok(1)
            }
            Err(err) => {
                tx.rollback();
                Err(ApiError::Database(err))
            }
        }
    }
}
