use std::process;

use actix_web::{App, HttpServer};
use mongodb::sync::{Client, Database};

use routes::{todo_controller, user_controller};

use crate::base::AppConfig;

mod base;
mod routes;
mod services;

pub struct AppState {
    db: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::new();
    if let Ok(client) = Client::with_uri_str(app_config.mongodb_uri) {
        let db = client.database(app_config.mongodb_database.as_str());
        let uri = format!("127.0.0.1:{}", app_config.port);
        println!("Server Start At : http://{}", uri);
        HttpServer::new(move || {
            App::new()
                .data(AppState { db: db.clone() })
                .service(user_controller::create_user)
                .service(user_controller::delete_user)
                .service(user_controller::update_user)
                .service(user_controller::find_user)
                .service(user_controller::find_user_list)
                .service(user_controller::do_login_by_password)
                .service(todo_controller::create_todo)
                .service(todo_controller::delete_todo)
                .service(todo_controller::update_todo)
                .service(todo_controller::find_todo)
        })
        .bind(uri)?
        .run()
        .await
    } else {
        println!("Failed to connect to MongoDB");
        process::exit(1);
    }
}
