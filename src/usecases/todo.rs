use crate::entities::todo::{Todo, TodoItem};
use crate::adapters::repositories::todo::TodoRepository;

pub trait TodoUseCase {
    fn create_todo(&mut self, title: String) -> Result<Todo, String>;
    fn get_todos(&self) -> Vec<Todo>;
    fn update_todo(&mut self, id: usize, completed: bool) -> Result<Todo, String>;
    fn delete_todo(&mut self, id: usize) -> Result<(), String>;
}

pub struct TodoUseCaseImpl<R: TodoRepository> {
    repository: R,
}

impl<R: TodoRepository> TodoUseCaseImpl<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: TodoRepository> TodoUseCase for TodoUseCaseImpl<R> {
    fn create_todo(&mut self, title: String) -> Result<Todo, String> {
        let id = self.repository.next_id();
        let todo = Todo::new(id, title);
        self.repository.save(&todo).map(|_| todo)
    }

    fn get_todos(&self) -> Vec<Todo> {
        self.repository.find_all()
    }

    fn update_todo(&mut self, id: usize, completed: bool) -> Result<Todo, String> {
        let mut todo = self.repository.find_by_id(id).ok_or("Todo not found")?;
        if completed {
            todo.complete();
        } else {
            todo.uncomplete();
        }
        self.repository.save(&todo).map(|_| todo)
    }

    fn delete_todo(&mut self, id: usize) -> Result<(), String> {
        self.repository.delete(id)
    }
}
