use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};

#[get("/api/v1/todos")]
async fn get_todos() -> impl Responder {
    HttpResponse::Ok().body("Get all todos")
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