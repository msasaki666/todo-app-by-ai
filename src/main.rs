use std::io;

struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String) {
        let todo = Todo {
            id: self.next_id,
            title,
            completed: false,
        };
        self.todos.push(todo);
        self.next_id += 1;
        println!("タスクが追加されました。");
    }

    fn edit_task(&mut self, id: usize, new_title: String) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.title = new_title;
            println!("タスクが編集されました。");
        } else {
            println!("指定されたIDのタスクが見つかりません。");
        }
    }

    fn complete_task(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            println!("タスクが完了しました。");
        } else {
            println!("指定されたIDのタスクが見つかりません。");
        }
    }

    fn delete_task(&mut self, id: usize) {
        self.todos.retain(|t| t.id != id);
        println!("タスクが削除されました。");
    }

    fn list_tasks(&self) {
        if self.todos.is_empty() {
            println!("タスクはありません。");
        } else {
            for todo in &self.todos {
                println!("ID: {}, タイトル: {}, 完了: {}", todo.id, todo.title, todo.completed);
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("\ntodoアプリへようこそ！");
        println!("1: タスクを追加");
        println!("2: タスクを編集");
        println!("3: タスクを完了");
        println!("4: タスクを削除");
        println!("5: タスク一覧を表示");
        println!("6: 終了");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("入力の読み取りに失敗しました");

        match choice.trim() {
            "1" => {
                println!("新しいタスクのタイトルを入力してください：");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("入力の読み取りに失敗しました");
                todo_list.add_task(title.trim().to_string());
            },
            "2" => {
                println!("編集するタスクのIDを入力してください：");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("入力の読み取りに失敗しました");
                let id: usize = id.trim().parse().expect("無効なIDです");

                println!("新しいタイトルを入力してください：");
                let mut new_title = String::new();
                io::stdin().read_line(&mut new_title).expect("入力の読み取りに失敗しました");
                todo_list.edit_task(id, new_title.trim().to_string());
            },
            "3" => {
                println!("完了するタスクのIDを入力してください：");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("入力の読み取りに失敗しました");
                let id: usize = id.trim().parse().expect("無効なIDです");
                todo_list.complete_task(id);
            },
            "4" => {
                println!("削除するタスクのIDを入力してください：");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("入力の読み取りに失敗しました");
                let id: usize = id.trim().parse().expect("無効なIDです");
                todo_list.delete_task(id);
            },
            "5" => {
                todo_list.list_tasks();
            },
            "6" => {
                println!("アプリケーションを終了します。");
                break;
            },
            _ => println!("無効な選択です。もう一度お試しください。"),
        }
    }
}
