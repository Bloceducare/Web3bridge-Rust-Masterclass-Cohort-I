use std::fmt;

#[derive(Clone)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

impl Todo {
    fn new(id: usize, title: &str) -> Self {
        Todo {
            id,
            title: title.to_string(),
            completed: false,
        }
    }

    fn update(&mut self, new_title: &str) {
        self.title = new_title.to_string();
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Todo {{ id: {}, title: '{}', completed: {} }}",
            self.id, self.title, self.completed
        )
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    // Create
    todos.push(Todo::new(1, "Learn Rust"));
    todos.push(Todo::new(2, "Build a project"));

    // Update
    if let Some(todo) = todos.iter_mut().find(|t| t.id == 1) {
        todo.update("Learn Rust basics");
        println!("Updated: {}", todo);
    }

    // Edit (same as update)
    if let Some(todo) = todos.iter_mut().find(|t| t.id == 2) {
        todo.update("Build a Rust project");
        println!("Edited: {}", todo);
    }

    // Mark as completed
    if let Some(todo) = todos.iter_mut().find(|t| t.id == 1) {
        todo.mark_completed();
        println!("Marked as completed: {}", todo);
    }

    // Delete
    if let Some(pos) = todos.iter().position(|t| t.id == 2) {
        let removed = todos.remove(pos);
        println!("Deleted: {}", removed);
    }

    // Print all
    println!("\nAll Todos:");
    for todo in &todos {
        println!("{}", todo);
    }
}