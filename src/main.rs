mod entities;
mod adapters;
mod usecases;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use adapters::repositories::todo::InMemoryTodoRepository;
use usecases::todo::TodoUseCaseImpl;
use adapters::controllers::todo::{TodoController, CreateTodo, UpdateTodo};
use std::sync::Mutex;
use actix_files::Files;

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("../static/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // リポジトリ、ユースケース、コントローラーの初期化
    let repository = InMemoryTodoRepository::new();
    let use_case = TodoUseCaseImpl::new(repository);
    let controller = web::Data::new(Mutex::new(TodoController::new(use_case)));

    // HTTPサーバーの設定と起動
    HttpServer::new(move || {
        App::new()
            .app_data(controller.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
            .service(
                web::scope("/todos")
                    .route("", web::post().to(|c: web::Data<Mutex<TodoController<TodoUseCaseImpl<InMemoryTodoRepository>>>>, form: web::Form<CreateTodo>| async move { c.lock().unwrap().create_todo(form).await }))
                    .route("", web::get().to(|c: web::Data<Mutex<TodoController<TodoUseCaseImpl<InMemoryTodoRepository>>>>| async move { c.lock().unwrap().get_todos().await }))
                    .route("/{id}", web::put().to(|c: web::Data<Mutex<TodoController<TodoUseCaseImpl<InMemoryTodoRepository>>>>, id: web::Path<usize>, todo: web::Json<UpdateTodo>| async move { c.lock().unwrap().update_todo(id, todo).await }))
                    .route("/{id}", web::delete().to(|c: web::Data<Mutex<TodoController<TodoUseCaseImpl<InMemoryTodoRepository>>>>, id: web::Path<usize>| async move { c.lock().unwrap().delete_todo(id).await }))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
