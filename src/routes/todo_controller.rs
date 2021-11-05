use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[post("/todo")]
pub async fn create_todo(todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().body(format!("create_todo ! id:{}", todo.id))
}

#[delete("/todo/{id}")]
pub async fn delete_todo(web::Path(id): web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("delete_todo ! id:{}", id))
}

#[put("/todo/{id}")]
pub async fn update_todo(web::Path(id): web::Path<u32>, todo: web::Json<Todo>) -> impl Responder {
    HttpResponse::Ok().body(format!("update_todo ! id:{}, title :{}", id, todo.title))
}

#[get("/todo/{id}")]
pub async fn find_todo(web::Path(id): web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("find_todo ! id:{}", id))
}
