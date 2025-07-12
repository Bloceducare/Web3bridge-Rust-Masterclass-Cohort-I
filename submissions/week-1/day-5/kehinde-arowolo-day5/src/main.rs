use std::io;

#[derive(Debug, Clone)]
struct TodoListItems {
    id: usize,
    description: String,
    completed: bool,
}

struct TodoManager {
    todos: Vec<TodoListItems>,
    next_id: usize,
}


impl TodoManager {
    fn new() -> Self {
        TodoManager {
            todos:  Vec::new(),
            next_id: 1,
        }
    }

    fn create_todo(&mut self, description:String) -> usize {
        let todo = TodoListItems {
            id: self.next_id,
            description,
            completed: false,
        };

        let id = todo.id;
        self.todos.push(todo);
        self.next_id += 1;
        println!("Todo List created with the ID: {}", id);
        id
    }

    fn update_todo(&mut self, id: usize, new_description: String) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.description = new_description;
            println!("Todo {} updated successfully", id);
            true
        } else {
            println!("Todo with ID {} not found", id);
            false
        }
    }

    fn delete_todo(&mut self, id: usize) -> bool {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(pos);
            println!("Todo {} deleted successfully", id);
            true
        } else {
            println!("Todo with ID {} not found", id);
            false
        }
    }

    fn edit_todo(&mut self, id: usize, new_description: String) -> bool {
        self.update_todo(id, new_description)
    }

    fn mark_completed(&mut self, id: usize) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            println!("Todo {} marked as completed", id);
            true
        } else {
            println!("Todo with ID {} not found", id);
            false
        }
    }

    fn list_todos(&self) {
        if self.todos.is_empty() {
            println!("No todos found");
            return;
        }
        
        println!("\n--- Todo List ---");
        for todo in &self.todos {
            let status = if todo.completed { "✓" } else { "○" };
            println!("{} [{}] {}", status, todo.id, todo.description);
        }
        println!();
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_id_input() -> usize {
    loop {
        let input = get_input("Enter todo ID:");
        match input.parse() {
            Ok(id) => return id,
            Err(_) => println!("Invalid ID. Please enter a number."),
        }
    }
}

fn main() {
    let mut todo_manager = TodoManager::new();
    
    loop {
        println!("\n--- Todo Manager ---");
        println!("1. Create Todo");
        println!("2. Update Todo");
        println!("3. Delete Todo");
        println!("4. Edit Todo");
        println!("5. Mark Todo as Completed");
        println!("6. List All Todos");
        println!("7. Exit");
        
        let choice = get_input("Choose an option (1-7):");
        
        match choice.as_str() {
            "1" => {
                let description = get_input("Enter todo description:");
                todo_manager.create_todo(description);
            },
            "2" => {
                todo_manager.list_todos();
                let id = get_id_input();
                let new_description = get_input("Enter new description:");
                todo_manager.update_todo(id, new_description);
            },
            "3" => {
                todo_manager.list_todos();
                let id = get_id_input();
                todo_manager.delete_todo(id);
            },
            "4" => {
                todo_manager.list_todos();
                let id = get_id_input();
                let new_description = get_input("Enter new description:");
                todo_manager.edit_todo(id, new_description);
            },
            "5" => {
                todo_manager.list_todos();
                let id = get_id_input();
                todo_manager.mark_completed(id);
            },
            "6" => {
                todo_manager.list_todos();
            },
            "7" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid option. Please choose 1-7."),
        }
    }
}
