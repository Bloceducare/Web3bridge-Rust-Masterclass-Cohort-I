#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    fn create_todo(&mut self, title: String) -> &Todo {
        let todo = Todo {
            id: self.todos.len() as u32 + 1,
            title,
            completed: false,
        };
        self.todos.push(todo.clone());

        println!("Created Todo: {:?}", todo);
        self.todos.last().unwrap()
    }

    fn update_todo(&mut self, id: u32, title: Option<String>, completed: Option<bool>) -> Option<&Todo> {
        for todo in &mut self.todos {
            if todo.id == id {
                let old_todo = todo.clone();
                if let Some(new_title) = title {
                    todo.title = new_title;
                }
                if let Some(new_completed) = completed {
                    todo.completed = new_completed;
                }
                println!("Updated Todo ID {}: {:?} -> {:?}", id, old_todo, todo);
                return Some(todo);
            }
        }
        println!("Failed to update Todo ID {}: Not found", id);
        None
    }

    fn delete_todo(&mut self, id: u32) -> Option<Todo> {
        if let Some(pos) = self.todos.iter().position(|todo| todo.id == id) {
            let deleted_todo = self.todos.remove(pos);
            println!("Deleted Todo: {:?}", deleted_todo);
            Some(deleted_todo)
        } else {
            println!("Failed to delete Todo ID {}: Not found", id);
            None
        }
    }

    fn get_todo(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    fn list_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    fn mark_completed(&mut self, id: u32) -> Option<&Todo> {
        for todo in &mut self.todos {
            if todo.id == id {
                todo.completed = true;
                println!("Marked Todo ID {} as completed: {:?}", id, todo);
                return Some(todo);
            }
        }
        println!("Failed to mark Todo ID {} as completed: Not found", id);
        None
    }

    fn edit_todo(&mut self, id: u32, new_title: String) -> Option<&Todo> {
        for todo in &mut self.todos {
            if todo.id == id {
                let old_title = todo.title.clone();
                todo.title = new_title;
                println!("Edited Todo ID {}: '{}' -> '{}'", id, old_title, todo.title);
                return Some(todo);
            }
        }
        println!("Failed to edit Todo ID {}: Not found", id);
        None
    }
}

fn main() {
    
    let mut todo_list = TodoList::new();
    
    // Create Todos
    println!("=== Creating Todos ===");
    let _todo1 = todo_list.create_todo("Learn Rust".to_string());
    let _todo2 = todo_list.create_todo("Build a web app".to_string());
    let _todo3 = todo_list.create_todo("Write documentation".to_string());
    let _todo4 = todo_list.create_todo("Deploy to production".to_string());
    
    println!("\n=== Current Todo List ===");
    for todo in todo_list.list_todos() {
        println!("{:?}", todo);
    }
    
    // Update Todo
    println!("\n=== Updating Todos ===");
    todo_list.update_todo(2, Some("Build a full-stack web application".to_string()), None);
    todo_list.update_todo(99, Some("Non-existent todo".to_string()), None); // This will fail
    
    // Edit Todo
    println!("\n=== Editing Todos ===");
    todo_list.edit_todo(1, "Master Rust Programming".to_string());
    todo_list.edit_todo(88, "Another non-existent todo".to_string()); // This will fail
    
    // Mark Todo as Completed
    println!("\n=== Marking Todos as Completed ===");
    todo_list.mark_completed(1);
    todo_list.mark_completed(3);
    todo_list.mark_completed(77); // This will fail
    
    // Delete Todo
    println!("\n=== Deleting Todos ===");
    todo_list.delete_todo(4);
    todo_list.delete_todo(55); // This will fail
    
    // Final state
    println!("\n=== Final Todo List ===");
    for todo in todo_list.list_todos() {
        let status = if todo.completed { "Done" } else { "In Progress" };
        println!("{} ID: {}, Title: '{}', Completed: {}", status, todo.id, todo.title, todo.completed);
    }
    
}
