use entities::todo::Todo;
use std::sync::{Arc, Mutex};

use crate::entities::{self, todo::TodoItem};

pub trait TodoRepository {
    fn next_id(&self) -> usize;
    fn save(&mut self, todo: &Todo) -> Result<(), String>;
    fn find_all(&self) -> Vec<Todo>;
    fn find_by_id(&self, id: usize) -> Option<Todo>;
    fn delete(&mut self, id: usize) -> Result<(), String>;
}

pub struct InMemoryTodoRepository {
    todos: Arc<Mutex<Vec<Todo>>>,
    next_id: Arc<Mutex<usize>>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

impl TodoRepository for InMemoryTodoRepository {
    fn next_id(&self) -> usize {
        let mut id = self.next_id.lock().unwrap();
        let current_id = *id;
        *id += 1;
        current_id
    }

    fn save(&mut self, todo: &Todo) -> Result<(), String> {
        let mut todos = self.todos.lock().unwrap();
        if let Some(existing_todo) = todos.iter_mut().find(|t| t.id() == todo.id()) {
            *existing_todo = todo.clone();
        } else {
            todos.push(todo.clone());
        }
        Ok(())
    }

    fn find_all(&self) -> Vec<Todo> {
        self.todos.lock().unwrap().clone()
    }

    fn find_by_id(&self, id: usize) -> Option<Todo> {
        self.todos
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.id() == id)
            .cloned()
    }

    fn delete(&mut self, id: usize) -> Result<(), String> {
        let mut todos = self.todos.lock().unwrap();
        let initial_len = todos.len();
        todos.retain(|t| t.id() != id);
        if todos.len() != initial_len {
            Ok(())
        } else {
            Err("Todo not found".to_string())
        }
    }
}
