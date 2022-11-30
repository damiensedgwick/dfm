use actix_cors::Cors;
use actix_web::{
    delete, get, http, http::header::ContentType, post, put, web, App, HttpResponse, HttpServer,
    Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteConnection;
use sqlx::Connection;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i64,
    title: String,
    completed: bool,
    archived: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct NewTodo {
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
async fn get_todo_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let mut conn = SqliteConnection::connect("sqlite:todos.db").await.unwrap();

    let row = sqlx::query_as!(TodoRow, "SELECT * FROM todos WHERE id =?", id)
        .fetch_one(&mut conn)
        .await;

    match row {
        Ok(row) => {
            let todo = Todo {
                id: row.id,
                title: row.title.clone(),
                completed: row.completed.unwrap_or(0) == 1,
                archived: row.archived.unwrap_or(0) == 1,
            };
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&todo).unwrap())
        }

        Err(_) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body("404: Todo not found"),
    }
}

#[post("/api/v1/todos")]
async fn create_todo(json: web::Json<NewTodo>) -> impl Responder {
    let mut conn = SqliteConnection::connect("sqlite:todos.db").await.unwrap();

    let query = sqlx::query!(
        "INSERT INTO todos (title, completed, archived) VALUES (?, ?, ?)",
        json.title,
        json.completed,
        json.archived,
    );

    let result = query.execute(&mut conn).await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("Todo created"),

        Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("500: Internal Server Error"),
    }
}

#[put("/api/v1/todos/{id}")]
async fn update_todo(path: web::Path<String>, json: web::Json<Todo>) -> impl Responder {
    let id = path.into_inner();

    let mut conn = SqliteConnection::connect("sqlite:todos.db").await.unwrap();

    let query = sqlx::query!(
        "UPDATE todos SET title = ?, completed = ?, archived = ? WHERE id = ?",
        json.title,
        json.completed,
        json.archived,
        id
    );

    let result = query.execute(&mut conn).await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("Todo successfully updated"),

        Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("500: Internal Server Error"),
    }
}

#[delete("/api/v1/todos/{id}")]
async fn delete_todo(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let mut conn = SqliteConnection::connect("sqlite:todos.db").await.unwrap();

    let query = sqlx::query!("DELETE FROM todos WHERE id = ?", id);

    let result = query.execute(&mut conn).await;

    match result {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("Todo has been deleted"),

        Err(_) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("500: Internal Server Error"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".localhost:5173"))
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
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
