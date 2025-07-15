#[derive(Debug, Clone)]
pub struct Todo {
    id: u32,
    description: String,
    completed: bool,
}

pub struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self, description: String) {
        let todo = Todo {
            id: self.next_id,
            description,
            completed: false,
        };
        self.todos.push(todo);
        self.next_id += 1;
    }

    pub fn update(&mut self, id: u32, new_description: String) -> Result<(), String> {
        let todo = self.todos.iter_mut().find(|todo| todo.id == id);
        match todo {
            Some(e) => {
                e.description = new_description;
                Ok(())
            }
            None => Err(format!("Todo with id {} not found", id)),
        }
    }

    pub fn delete(&mut self, id: u32) -> Result<(), String> {
        let index = self.todos.iter().position(|todo| todo.id == id);
        match index {
            Some(i) => {
                self.todos.remove(i);
                Ok(())
            }
            None => Err(format!("Todo with id {} not found", id)),
        }
    }

    pub fn is_completed(&mut self, id: u32) -> Result<(), String> {
        match self.todos.iter_mut().find(|todo| todo.id == id) {
            Some(todo) => {
                todo.completed = true;
                Ok(())
            }
            None => Err(format!("Todo with id {} not found", id)),
        }
    }

    pub fn print_all(&self) {
        for todo in &self.todos {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                todo.id, todo.description, todo.completed
            );
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    // create to do lists
    todo_list.create("Learn Rust".to_string());
    todo_list.create("Build Rust Defi protocol product".to_string());
    todo_list.create("Deploy to production".to_string());

    // Print all todos
    todo_list.print_all();

    println!(
        "****************************************************************************************************"
    );

    // Update a todo
    todo_list
        .update(2, "Build Rust lending protocol product".to_string())
        .unwrap();

    // print all todos
    todo_list.print_all();

    println!(
        "****************************************************************************************************"
    );

    // Complete a todo
    todo_list.is_completed(1).unwrap();

    // Delete a todo
    todo_list.delete(2).unwrap();

    // Print all remaining todos
    todo_list.print_all();
}
