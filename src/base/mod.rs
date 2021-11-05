use std::env;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub struct AppConfig {
    pub port: String,
    pub mongodb_uri: String,
    pub mongodb_database: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            port: env::var("APP_PORT").unwrap_or("4300".to_string()),
            mongodb_uri: env::var("MONGODB_URI").unwrap_or("mongodb://localhost:27017".to_string()),
            mongodb_database: env::var("MONGODB_DATABASE").unwrap_or("todo-app".to_string()),
        }
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
