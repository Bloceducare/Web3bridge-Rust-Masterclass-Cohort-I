#[derive(Debug, Clone)]
struct Todo {
    id: usize,
    title: String,
    description: String,
    completed: bool,
}

impl Todo {
    fn create(id:usize, title: String, description: String) -> Self {
        Todo {
            id,
            title,
            description,
            completed: false,
        }
    }

    fn mark_as_completed(&mut self) {
        self.completed = true;
    }

    fn mark_as_incomplete(&mut self) {
        self.completed = false;
    }

    fn update(&mut self, title: Option<String>, description: Option<String>) {
        if let Some(title) = title {
            self.title = title;
        }
        if let Some(description) = description {
            self.description = description;
        }
    }
}

struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    fn add_todo(&mut self, title: String, description: String) -> usize {
        let todo = Todo::create(self.next_id, title, description);
        let id = todo.id;
        self.todos.push(todo);
        self.next_id += 1;
        println!("Added todo #{}: {}", id, self.todos.last().unwrap().title);
        id
    }

    fn get_todo(&self, id: usize) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    fn get_todo_mut(&mut self, id: usize) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    fn update_todo(&mut self, id: usize, title: Option<String>, description: Option<String>) -> bool {
        if let Some(todo) = self.get_todo_mut(id) {
            todo.update(title, description);
            println!("Updated todo #{}", id);
            true
        } else {
            println!("Todo with id {} not found", id);
            false
        }
    }

    fn mark_completed(&mut self, id: usize) -> bool {
        if let Some(todo) = self.get_todo_mut(id) {
            todo.mark_as_completed();
            println!("Marked todo #{} as completed", id);
            true
        } else {
            println!("Todo with id {} not found", id);
            false
        }
    }

    fn mark_incomplete(&mut self, id: usize) -> bool {
        if let Some(todo) = self.get_todo_mut(id) {
            todo.mark_as_incomplete();
            println!("Marked todo #{} as incomplete", id);
            true
        } else {
            println!("Todo with id {} not found", id);
            false
        }
    }

    fn delete_todo(&mut self, id: usize) -> bool {
        if let Some(index) = self.todos.iter().position(|todo| todo.id == id) {
            let removed_todo = self.todos.remove(index);
            println!("Deleted todo #{}: '{}'", removed_todo.id, removed_todo.title);
            true
        } else {
            println!("Todo with id {} not found", id);
            false
        }
    }

    fn list_todos(&self) {
        if self.todos.is_empty() {
            println!("No todos found!");
            return;
        }

        println!("\n=== Todo List ===");
        for todo in &self.todos {
            let status = if todo.completed { "✓" } else { "○" };
            println!("{} #{}: {} - {}", status, todo.id, todo.title, todo.description);
        }
        println!("================\n");
    }

    fn list_completed(&self) {
        let completed: Vec<&Todo> = self.todos.iter().filter(|todo| todo.completed).collect();
        
        if completed.is_empty() {
            println!("No completed todos!");
            return;
        }

        println!("\n=== Completed Todos ===");
        for todo in completed {
            println!("✓ #{}: {} - {}", todo.id, todo.title, todo.description);
        }
        println!("=======================\n");
    }

    fn list_pending(&self) {
        let pending: Vec<&Todo> = self.todos.iter().filter(|todo| !todo.completed).collect();
        
        if pending.is_empty() {
            println!("No pending todos!");
            return;
        }

        println!("\n=== Pending Todos ===");
        for todo in pending {
            println!("○ #{}: {} - {}", todo.id, todo.title, todo.description);
        }
        println!("=====================\n");
    }

    fn get_stats(&self) {
        let total = self.todos.len();
        let completed = self.todos.iter().filter(|todo| todo.completed).count();
        let pending = total - completed;
        
        println!("\n=== Todo Statistics ===");
        println!("Total todos: {}", total);
        println!("Completed: {}", completed);
        println!("Pending: {}", pending);
        if total > 0 {
            let completion_rate = (completed as f64 / total as f64) * 100.0;
            println!("Completion rate: {:.1}%", completion_rate);
        }
        println!("=======================\n");
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    let id1 = todo_list.add_todo(
        "Learn Rust".to_string(),
        "Complete the Rust tutorial".to_string(),
    );
    
    let id2 = todo_list.add_todo(
        "Build a project".to_string(),
        "Create a todo app in Rust".to_string(),
    );
    
    let id3 = todo_list.add_todo(
        "Master Rust".to_string(),
        "Become proficient in Rust programming".to_string(),
    );

    todo_list.list_todos();

    todo_list.mark_completed(id1);
    todo_list.mark_completed(id2);

    todo_list.update_todo(
        id3,
        Some("Master Advanced Rust".to_string()),
        Some("Become expert in advanced Rust concepts".to_string())
    );

    todo_list.list_todos();

    todo_list.list_completed();

    todo_list.list_pending();

    todo_list.get_stats();

    todo_list.delete_todo(id2);

    println!("Final todo list:");
    todo_list.list_todos();
    todo_list.get_stats();
}
