use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::usecases::todo::TodoUseCase;

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub completed: bool,
}

pub struct TodoController<U: TodoUseCase> {
    use_case: U,
}

impl<U: TodoUseCase> TodoController<U> {
    pub fn new(use_case: U) -> Self {
        Self { use_case }
    }

    pub async fn create_todo(&mut self, form: web::Form<CreateTodo>) -> impl Responder {
        match self.use_case.create_todo(form.title.clone()) {
            Ok(todo) => HttpResponse::Ok().json(todo),
            Err(e) => HttpResponse::InternalServerError().body(e),
        }
    }

    pub async fn get_todos(&self) -> impl Responder {
        let todos = self.use_case.get_todos();
        HttpResponse::Ok().json(todos)
    }

    pub async fn update_todo(
        &mut self,
        id: web::Path<usize>,
        todo: web::Json<UpdateTodo>,
    ) -> impl Responder {
        match self.use_case.update_todo(*id, todo.completed) {
            Ok(updated_todo) => HttpResponse::Ok().json(updated_todo),
            Err(e) => HttpResponse::NotFound().body(e),
        }
    }

    pub async fn delete_todo(&mut self, id: web::Path<usize>) -> impl Responder {
        match self.use_case.delete_todo(*id) {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::NotFound().body(e),
        }
    }
}
