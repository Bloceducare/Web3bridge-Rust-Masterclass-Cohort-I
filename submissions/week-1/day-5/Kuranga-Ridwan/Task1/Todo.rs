
#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
    edited: bool, // To track if the todo has been edited
}

// A simple structure to manage the list of todos.
struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    // Creates a new, empty TodoList.
    fn new() -> Self {
        TodoList {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    // Create a new todo and add it to the list.
    fn create_todo(&mut self, text: &str) -> &Todo {
        let todo = Todo {
            id: self.next_id,
            text: String::from(text),
            completed: false,
            edited: false,
        };
        self.todos.push(todo);
        self.next_id += 1;
        // Return a reference to the newly created todo.
        self.todos.last().unwrap()
    }

    // Find a todo by its ID and mark it as completed.
    fn mark_todo_as_completed(&mut self, id: u32) -> Option<&Todo> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            Some(todo)
        } else {
            None // Return None if no todo with the given id is found.
        }
    }

    // Find a todo by its ID and update its text.
    // This function is used for both "updating" and "editing".
    fn edit_todo(&mut self, id: u32, new_text: &str) -> Option<&Todo> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.text = String::from(new_text);
            todo.edited = true;
            Some(todo)
        } else {
            None
        }
    }

    
    fn update_todo(&mut self, id: u32, new_text: &str) -> Option<&Todo> {
        self.edit_todo(id, new_text)
    }

    // Delete a todo from the list by its ID.
    fn delete_todo(&mut self, id: u32) -> bool {
        let initial_len = self.todos.len();
        // Retain only the todos that do NOT have the specified id.
        self.todos.retain(|t| t.id != id);
        // Return true if a todo was deleted.
        self.todos.len() < initial_len
    }

    
    fn print_todos(&self) {
        println!("\n--- To-Do List ---");
        for todo in &self.todos {
            let status = if todo.completed { "[x]" } else { "[ ]" };
            let edited_status = if todo.edited { "(edited)" } else { "" };
            println!(
                "{} ID: {} - {} {}",
                status, todo.id, todo.text, edited_status
            );
        }
        println!("--------------------\n");
    }
}

fn main() {
    // Create a new todo list manager.
    let mut my_todos = TodoList::new();

    println!("Starting with an empty To-Do list.");
    my_todos.print_todos();

    // 1. Create Todos
    println!("Creating two new todos...");
    my_todos.create_todo("Learn Rust");
    my_todos.create_todo("Build a project");
    my_todos.print_todos();

    // 2. Mark a todo as completed
    println!("Marking 'Learn Rust' (ID: 1) as completed...");
    my_todos.mark_todo_as_completed(1);
    my_todos.print_todos();

    // 3. Edit a todo
    println!("Editing 'Build a project' (ID: 2)...");
    my_todos.edit_todo(2, "Build an awesome Rust project");
    my_todos.print_todos();

    // 4. Update a todo (using the update function)
    println!("Updating 'Learn Rust' (ID: 1) text...");
    my_todos.update_todo(1, "Master Rust concepts");
    my_todos.print_todos();

    // 5. Delete a todo
    println!("Deleting the completed todo (ID: 1)...");
    my_todos.delete_todo(1);
    my_todos.print_todos();

    // Try to delete a non-existent todo
    println!("Trying to delete a non-existent todo (ID: 99)...");
    if !my_todos.delete_todo(99) {
        println!("Todo with ID 99 not found.");
    }
    my_todos.print_todos();
}
