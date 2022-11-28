use actix_web::{delete, get, http::header::ContentType, post, put, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnection};
use sqlx::Connection;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i64,
    title: String,
    completed: bool,
    archived: bool,
}

#[derive(sqlx::FromRow)]
struct TodoRow {
    id: i64,
    title: String,
    completed: Option<i64>,
    archived: Option<i64>,
}

#[get("/api/v1/todos")]
async fn get_todos() -> impl Responder {
    let mut conn = SqliteConnection::connect("sqlite:todos.db").await.unwrap();

    let rows = sqlx::query_as!(TodoRow, "SELECT * FROM todos")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    let todos = rows
        .into_iter()
        .map(|row| Todo {
            id: row.id,
            title: row.title.clone(),
            completed: row.completed.unwrap_or(0) == 1,
            archived: row.archived.unwrap_or(0) == 1,
        })
        .collect::<Vec<Todo>>();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&todos).unwrap())
}

#[get("/api/v1/todos/{id}")]
async fn get_todo_by_id() -> impl Responder {
    HttpResponse::Ok().body("Get a todo by id")
}

#[post("/api/v1/todos")]
async fn create_todo() -> impl Responder {
    HttpResponse::Ok().body("Create a todo")
}

#[put("/api/v1/todos/{id}")]
async fn update_todo() -> impl Responder {
    HttpResponse::Ok().body("Update a todo")
}

#[delete("/api/v1/todos/{id}")]
async fn delete_todo() -> impl Responder {
    HttpResponse::Ok().body("Delete a todo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_todos)
            .service(get_todo_by_id)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
