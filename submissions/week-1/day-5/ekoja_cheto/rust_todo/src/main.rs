use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

struct TodoList {
    todos: HashMap<u32, Todo>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    fn create_todo(&mut self, title: String) {
        let todo = Todo {
            id: self.next_id,
            title,
            completed: false,
        };
        self.todos.insert(self.next_id, todo);
        println!(" Todo Created: {:?}", self.todos.get(&self.next_id));
        self.next_id += 1;
    }

    fn edit_todo(&mut self, id: u32, new_title: String) {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.title = new_title;
            println!(" Todo Edited: {:?}", todo);
        } else {
            println!(" Todo with id {} not found", id);
        }
    }

    fn mark_completed(&mut self, id: u32) {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.completed = true;
            println!(" Todo Marked Completed: {:?}", todo);
        } else {
            println!(" Todo with id {} not found", id);
        }
    }

    fn delete_todo(&mut self, id: u32) {
        if self.todos.remove(&id).is_some() {
            println!("Todo with id {} deleted", id);
        } else {
            println!(" Todo with id {} not found", id);
        }
    }

    fn list_todos(&self) {
        println!(" Current Todos:");
        for todo in self.todos.values() {
            println!("{:?}", todo);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.create_todo("Learn Rust".to_string());
    todo_list.create_todo("Build a Todo App".to_string());

    todo_list.edit_todo(1, "Learn Advanced Rust".to_string());

    todo_list.mark_completed(1);

    todo_list.delete_todo(2);

    todo_list.list_todos();
}
