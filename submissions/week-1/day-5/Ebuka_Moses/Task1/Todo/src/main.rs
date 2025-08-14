use std::collections::HashMap;

#[derive(Debug, Clone)]
enum TodoStatus {
    Pending,
    Completed,
}

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    status: TodoStatus,
    created_at: String,
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

    // Create Todo
    fn create_todo(&mut self, title: String, description: String) -> u32 {
        let todo = Todo {
            id: self.next_id,
            title: title.clone(),
            description: description.clone(),
            status: TodoStatus::Pending,
            created_at: "2024-01-15".to_string(), // Simplified for demo
        };
        
        println!("✅ Created Todo: {}", title);
        self.todos.insert(self.next_id, todo);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    // Update Todo
    fn update_todo(&mut self, id: u32, new_title: String, new_description: String) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            let old_title = todo.title.clone();
            todo.title = new_title.clone();
            todo.description = new_description.clone();
            println!("🔄 Updated Todo {}: '{}' -> '{}'", id, old_title, new_title);
            true
        } else {
            println!("❌ Todo with ID {} not found", id);
            false
        }
    }

    // Delete Todo
    fn delete_todo(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.remove(&id) {
            println!("🗑️  Deleted Todo {}: '{}'", id, todo.title);
            true
        } else {
            println!("❌ Todo with ID {} not found", id);
            false
        }
    }

    // Edit Todo (alias for update)
    fn edit_todo(&mut self, id: u32, new_title: String, new_description: String) -> bool {
        self.update_todo(id, new_title, new_description)
    }

    // Mark todo as completed
    fn mark_completed(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.status = TodoStatus::Completed;
            println!("✅ Marked Todo {} as completed: '{}'", id, todo.title);
            true
        } else {
            println!("❌ Todo with ID {} not found", id);
            false
        }
    }

    // Display all todos
    fn display_todos(&self) {
        if self.todos.is_empty() {
            println!("📝 No todos found!");
            return;
        }

        println!("\n📋 TODO LIST:");
        println!("{:-<60}", "");
        for todo in self.todos.values() {
            let status_icon = match todo.status {
                TodoStatus::Pending => "⏳",
                TodoStatus::Completed => "✅",
            };
            println!("{} [{}] {} - {}", status_icon, todo.id, todo.title, todo.description);
        }
        println!("{:-<60}", "");
    }
}

fn main() {
    println!("🚀 Starting Todo List Application\n");
    
    let mut todo_list = TodoList::new();

    // Create todos
    println!("=== CREATING TODOS ===");
    let todo1_id = todo_list.create_todo(
        "Learn Rust".to_string(),
        "Complete the Rust masterclass".to_string()
    );
    
    let todo2_id = todo_list.create_todo(
        "Build Web3 Project".to_string(),
        "Create a decentralized application".to_string()
    );
    
    let todo3_id = todo_list.create_todo(
        "Write Documentation".to_string(),
        "Document the project features".to_string()
    );

    // Display todos
    todo_list.display_todos();

    // Update a todo
    println!("\n=== UPDATING TODO ===");
    todo_list.update_todo(
        todo1_id,
        "Master Rust Programming".to_string(),
        "Complete advanced Rust concepts".to_string()
    );

    // Mark a todo as completed
    println!("\n=== MARKING TODO AS COMPLETED ===");
    todo_list.mark_completed(todo2_id);

    // Edit a todo
    println!("\n=== EDITING TODO ===");
    todo_list.edit_todo(
        todo3_id,
        "Write Comprehensive Documentation".to_string(),
        "Create detailed project documentation with examples".to_string()
    );

    // Display updated todos
    todo_list.display_todos();

    // Delete a todo
    println!("\n=== DELETING TODO ===");
    todo_list.delete_todo(todo1_id);

    // Final display
    todo_list.display_todos();

    println!("\n🎉 Todo List Application Demo Complete!");
}
