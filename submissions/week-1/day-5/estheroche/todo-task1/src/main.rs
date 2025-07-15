use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    description: String,
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

    fn create_todo(&mut self, title: &str, description: &str) {
        let todo = Todo {
            id: self.next_id,
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
        };
        self.todos.insert(self.next_id, todo.clone());
        // ✅ Explicitly read and print the `id`
        println!(
            "Created Todo [ID: {}] Title: '{}', Description: '{}'",
            todo.id, todo.title, todo.description
        );
        self.next_id += 1;
    }

    fn edit_todo(&mut self, id: u32, new_title: &str, new_description: &str) {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.title = new_title.to_string();
            todo.description = new_description.to_string();
            println!("Edited Todo [ID: {}]: {:?}", todo.id, todo);
        } else {
            println!("Todo with ID {} not found", id);
        }
    }

    fn delete_todo(&mut self, id: u32) {
        if self.todos.remove(&id).is_some() {
            println!("Deleted Todo with ID {}", id);
        } else {
            println!("Todo with ID {} not found", id);
        }
    }

    fn update_todo(&mut self, id: u32, title: &str, description: &str, completed: bool) {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.title = title.to_string();
            todo.description = description.to_string();
            todo.completed = completed;
            println!("Updated Todo [ID: {}]: {:?}", todo.id, todo);
        } else {
            println!("Todo with ID {} not found", id);
        }
    }

    fn mark_completed(&mut self, id: u32) {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.completed = true;
            println!("Marked as completed Todo [ID: {}]: {:?}", todo.id, todo);
        } else {
            println!("Todo with ID {} not found", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.create_todo("Learn Rust", "Finish the ownership chapter");
    todo_list.create_todo("Build a project", "Create a simple CLI app");

    todo_list.edit_todo(
        1,
        "Learn Rust in depth/Advance",
        "Master ownership and borrowing",
    );

    todo_list.mark_completed(2);

    todo_list.update_todo(1, "Learn Stylus Rust", "Smart Contract with Rust", true);

    todo_list.delete_todo(2);
}
