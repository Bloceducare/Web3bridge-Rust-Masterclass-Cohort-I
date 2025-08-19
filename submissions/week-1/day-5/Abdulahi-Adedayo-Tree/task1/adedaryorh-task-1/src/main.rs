use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl Todo {
    fn new(id: u32, title: String, description: String) -> Self {
        Todo {
            id,
            title,
            description,
            completed: false,
        }
    }
}

struct TodoManager {
    todos: HashMap<u32, Todo>,
    next_id: u32,
}

impl TodoManager {
    fn new() -> Self {
        TodoManager {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    fn create_todo(&mut self, title: String, description: String) -> u32 {
        let id = self.next_id;
        let todo = Todo::new(id, title, description);
        self.todos.insert(id, todo.clone());
        self.next_id += 1;
        println!("✅ Created todo with ID {}: '{}'", id, todo.title);
        id
    }

    fn update_todo(&mut self, id: u32, new_title: String, new_description: String) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            println!("🔄 Updating todo ID {}: '{}' -> '{}'", id, todo.title, new_title);
            todo.title = new_title;
            todo.description = new_description;
            true
        } else {
            println!("❌ Todo with ID {} not found", id);
            false
        }
    }

   
    fn delete_todo(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.remove(&id) {
            println!("🗑️ Deleted todo ID {}: '{}'", id, todo.title);
            true
        } else {
            println!("❌ Todo with ID {} not found", id);
            false
        }
    }


    fn edit_todo(&mut self, id: u32, new_title: String, new_description: String) -> bool {
        println!("📝 Editing todo ID {}...", id);
        self.update_todo(id, new_title, new_description)
    }

    fn mark_completed(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.completed = true;
            println!("✅ Marked todo ID {} as completed: '{}'", id, todo.title);
            true
        } else {
            println!("❌ Todo with ID {} not found", id);
            false
        }
    }

    fn display_todos(&self) {
        println!("\n📋 Current Todos:");
        println!("─────────────────────────────────────────");
        if self.todos.is_empty() {
            println!("No todos available.");
        } else {
            for (id, todo) in &self.todos {
                let status = if todo.completed { "✅" } else { "⏳" };
                println!("{} ID: {} | Title: '{}' | Description: '{}'", 
                         status, id, todo.title, todo.description);
            }
        }
        println!("─────────────────────────────────────────\n");
    }
}

fn main() {
    println!("🚀 Todo List Application");
    println!("========================\n");

    let mut todo_manager = TodoManager::new();

  
    println!("📝 Creating todos...");
    let id1 = todo_manager.create_todo(
        "Learn Rust".to_string(),
        "Complete the Rust programming tutorial".to_string()
    );
    let id2 = todo_manager.create_todo(
        "Build a project".to_string(),
        "Create a todo application in Rust".to_string()
    );
    let id3 = todo_manager.create_todo(
        "Write tests".to_string(),
        "Add unit tests for the todo application".to_string()
    );

    todo_manager.display_todos();

   
    println!("🔄 Updating a todo...");
    todo_manager.update_todo(
        id1,
        "Master Rust".to_string(),
        "Complete advanced Rust programming concepts".to_string()
    );

    todo_manager.display_todos();

   
    println!("📝 Editing a todo...");
    todo_manager.edit_todo(
        id3,
        "Write comprehensive tests".to_string(),
        "Add unit tests and integration tests for the todo application".to_string()
    );

    todo_manager.display_todos();

    println!("✅ Marking a todo as completed...");
    todo_manager.mark_completed(id2);

    todo_manager.display_todos();

   
    println!("🗑️ Deleting a todo...");
    todo_manager.delete_todo(id1);

    todo_manager.display_todos();

   
    println!("🧪 Testing error handling...");
    todo_manager.update_todo(999, "Non-existent".to_string(), "This should fail".to_string());
    todo_manager.mark_completed(999);
    todo_manager.delete_todo(999);

    println!("\n🎉 Todo application demo completed!");
}