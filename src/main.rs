use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    completed: bool,
}

struct AppState {
    todo_list: Mutex<Vec<Todo>>,
    next_id: Mutex<usize>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("../static/index.html"))
}

async fn create_todo(data: web::Data<AppState>, form: web::Form<CreateTodo>) -> impl Responder {
    let mut todo_list = data.todo_list.lock().unwrap();
    let mut next_id = data.next_id.lock().unwrap();

    let new_todo = Todo {
        id: *next_id,
        title: form.title.clone(),
        completed: false,
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    todo_list.push(new_todo.clone());
    *next_id += 1;

    HttpResponse::Ok().json(new_todo)
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todo_list = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(&*todo_list)
}

async fn update_todo(data: web::Data<AppState>, id: web::Path<usize>, todo: web::Json<UpdateTodo>) -> impl Responder {
    let mut todo_list = data.todo_list.lock().unwrap();
    if let Some(t) = todo_list.iter_mut().find(|t| t.id == *id) {
        t.completed = todo.completed;
        HttpResponse::Ok().json(t)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_todo(data: web::Data<AppState>, id: web::Path<usize>) -> impl Responder {
    let mut todo_list = data.todo_list.lock().unwrap();
    let initial_len = todo_list.len();
    todo_list.retain(|t| t.id != *id);
    if todo_list.len() != initial_len {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        todo_list: Mutex::new(Vec::new()),
        next_id: Mutex::new(1),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/todos", web::post().to(create_todo))
            .route("/todos", web::get().to(get_todos))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
