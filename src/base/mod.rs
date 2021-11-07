use std::env;

use mongodb::sync::{Client, Database};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub struct AppConfig {
    pub port: String,
    pub mongodb_uri: String,
    pub mongodb_database: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        let port = env::var("APP_PORT").unwrap_or("4300".to_string());
        let mongodb_uri =
            env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".to_string());
        let mongodb_database = env::var("MONGODB_DATABASE").unwrap_or("todo-app".to_string());
        AppConfig {
            port,
            mongodb_uri,
            mongodb_database,
        }
    }

    pub fn get_database(&self) -> Option<Database> {
        if let Ok(client) = Client::with_uri_str(self.mongodb_uri.as_str()) {
            let db = client.database(self.mongodb_database.as_str());
            return Some(db);
        }
        None
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiHeader {
    pub authorization: Option<String>,
    pub cookie: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    code: u32,
    data: T,
    msg: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> ApiResponse<T>
    where
        T: Serialize,
    {
        ApiResponse {
            code: 0,
            data,
            msg: String::from(""),
        }
    }
}

impl ApiResponse<String> {
    pub fn error(code: u32, msg: &str) -> ApiResponse<String> {
        ApiResponse {
            code,
            data: "".to_string(),
            msg: msg.to_string(),
        }
    }
}
