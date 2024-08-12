use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub trait TodoItem {
    fn new(id: usize, title: String) -> Self;
    fn id(&self) -> usize;
    fn title(&self) -> &str;
    fn is_completed(&self) -> bool;
    fn complete(&mut self);
    fn uncomplete(&mut self);
    fn created_at(&self) -> u64;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    title: String,
    completed: bool,
    created_at: u64,
}

impl TodoItem for Todo {
    fn new(id: usize, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn is_completed(&self) -> bool {
        self.completed
    }

    fn complete(&mut self) {
        self.completed = true;
    }

    fn uncomplete(&mut self) {
        self.completed = false;
    }

    fn created_at(&self) -> u64 {
        self.created_at
    }
}
