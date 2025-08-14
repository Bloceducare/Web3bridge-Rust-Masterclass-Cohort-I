#[derive(Debug, Clone)]
pub struct TodoItem {
    id: usize,
    title: String,
    description: String,
    is_done: bool,
}

pub struct TodoApp {
    pub todos: Vec<TodoItem>,
    next_id: usize,
}

impl TodoApp {
    pub fn new() -> Self {
        TodoApp { 
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_todo(&mut self, title: String, description: String) -> TodoItem {
        let todo = TodoItem {
            id: self.next_id,
            title,
            description,
            is_done: false,
        };

        self.todos.push(todo.clone());
        self.next_id += 1;
        
        println!("Created todo: {}", todo.title);
        todo
    }

  
    pub fn update_todo(&mut self, id: usize, new_title: String, new_description: String) {
        for todo in &mut self.todos {
            if todo.id == id {
                let old_title = todo.title.clone();
                todo.title = new_title;
                todo.description = new_description;
                println!("Updated todo ID {}: '{}' -> '{}'", id, old_title, todo.title);
                return;
            }
        }
        println!("Todo with ID {} not found", id);
    }

    pub fn delete_todo(&mut self, id: usize) {
        for i in 0..self.todos.len() {
            if self.todos[i].id == id {
                let deleted_todo = self.todos.remove(i);
                println!(" Deleted todo: '{}'", deleted_todo.title);
                return;
            }
        }
        println!("Todo with ID {} not found", id);
    }

    pub fn mark_completed(&mut self, id: usize) {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.is_done = true;
                println!("Marked todo '{}' as completed", todo.title);
                return;
            }
        }
        println!("Todo with ID {} not found", id);
    }

    pub fn mark_incomplete(&mut self, id: usize) {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.is_done = false;
                println!(" Marked todo '{}' as incomplete", todo.title);
                return;
            }
        }
        println!(" Todo with ID {} not found", id);
    }

    pub fn display_todos(&self) {
        println!("\n Current Todo List:");
        println!("====================");
        
        if self.todos.is_empty() {
            println!("No todos found.");
            return;
        }

        for todo in &self.todos {
            let status = if todo.is_done { "✅" } else { "⏳" };
            println!("{} [ID: {}] {}", status, todo.id, todo.title);
            println!("   Description: {}", todo.description);
            println!("   Status: {}", if todo.is_done { "Completed" } else { "Pending" });
            println!();
        }
    }
}

fn main() {
    let mut todo_app = TodoApp::new();

    println!("1. Creating todos...");
    todo_app.create_todo(
        String::from("Learn Rust"),
        String::from("Study rust programming language basics")
    );
    
    todo_app.create_todo(
        String::from("Build Todo App"),
        String::from("Create a working todo app in rust")
    );

    todo_app.create_todo(
        String::from("Do house chores"),
        String::from("Clean the whole house, and fix breakfast")
    );
    
    todo_app.display_todos();

    println!("2. Updating a todo...");
    todo_app.update_todo(
        1,
        String::from("Master Rust"),
        String::from("Become cracked in rust programming language")
    );

    println!("\n3. Marking todos as completed...");
    todo_app.mark_completed(1);
    todo_app.mark_completed(2);

    todo_app.display_todos();

    println!("4. Marking a todo as incomplete...");
    todo_app.mark_incomplete(1);

    println!("\n5. Deleting a todo...");
    todo_app.delete_todo(1);

    todo_app.display_todos();
}

