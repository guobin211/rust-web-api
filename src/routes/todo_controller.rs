use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

/// Authorization
pub async fn handle_todo(req: HttpRequest) -> impl Responder {
    let header = req.headers();
    if let Some(auth) = header.get("Authorization") {
        if auth.len() > 30 {
            return HttpResponse::Ok().json(vec![
                Todo {
                    id: 1,
                    title: "Learn Rust".to_string(),
                    completed: false,
                },
                Todo {
                    id: 2,
                    title: "Learn Actix".to_string(),
                    completed: false,
                },
            ]);
        }
    }
    let ok = StatusCode::from_u16(403).unwrap();
    HttpResponse::new(ok)
}
