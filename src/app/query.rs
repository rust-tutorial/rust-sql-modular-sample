use std::any::Any;
use std::collections::HashMap;
use std::fmt::Display;

use serde_json::Value;

pub fn query_load<T>(table: String, id: T) -> String where T: Display {
    format!("select id, username, email, phone, date_of_birth from {} where id = {}", table, id)
}

pub fn query_create(table: String) -> String {
    format!("insert into {} (id, username, email, phone, date_of_birth) values (?, ?, ?, ?, ?)", table)
}

pub fn query_update(table: String) -> String {
    format!("update {} set username = ?, email = ?, phone = ?, date_of_birth = ? where id = ?", table)
}

pub fn query_delete(table: String) -> String {
    format!("delete from {} where id = ?", table)
}

pub fn query_patch(table: String, id: String, m: HashMap<String, Value>) -> String {
    let mut query = format!("update {}", table);
    for k in m {
        let token = format!(" set {}='{}'", k.0, k.1);
        query.push_str(token.as_str());
    };
    let cont = format!(" where id='{}'", id.clone());
    query.push_str(cont.as_str());
    println!("Query: {}", query);
    query
}

