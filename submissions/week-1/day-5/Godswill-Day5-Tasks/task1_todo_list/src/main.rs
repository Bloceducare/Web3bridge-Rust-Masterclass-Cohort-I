// Task 1: Todo List in Rust
// This program demonstrates struct creation, vector manipulation, and function organization

// First, let's import what we need
use std::fmt; // For custom display formatting

// Define our Todo struct - this is like a blueprint for creating todo items
// Each field has a specific type and purpose
#[derive(Debug, Clone)] // These are "traits" that give our struct special abilities
struct Todo {
    id: u32,           // Unique identifier - u32 means unsigned 32-bit integer (0 to 4,294,967,295)
    title: String,     // The task description - String is owned, growable text
    completed: bool,   // Status - bool can be true or false
}

// Implement Display trait for our Todo struct
// This allows us to control how a Todo looks when printed
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The ? operator is for error handling - if write! fails, it returns the error
        write!(
            f,
            " ID: {} | Title: {} | Status: {}",
            self.id,
            self.title,
            if self.completed { " Completed" } else { " Pending" }
        )
    }
}

// Implementation block - this is where we define methods for our Todo struct
impl Todo {
    // Constructor function - creates a new Todo
    // 'new' is a Rust convention for constructors
    fn new(id: u32, title: String) -> Self {
        // Self refers to the Todo struct
        // We return a new instance with default values
        Todo {
            id,
            title,
            completed: false, // New todos start as not completed
        }
    }
}

// Function to create a new todo and add it to the list
// &mut Vec<Todo> means we're borrowing a mutable reference to the vector
// This allows us to modify the vector without taking ownership
fn create_todo(todos: &mut Vec<Todo>, id: u32, title: String) {
    println!("\n🔨 Creating new todo with ID: {} and title: '{}'", id, title);
    
    // Check if ID already exists to prevent duplicates
    if todos.iter().any(|todo| todo.id == id) {
        println!(" Error: Todo with ID {} already exists!", id);
        return; // Exit early if ID exists
    }
    
    // Create new todo using our constructor
    let new_todo = Todo::new(id, title);
    
    // Push the new todo to the vector
    // push() adds an element to the end of the vector
    todos.push(new_todo);
    
    println!(" Todo created successfully!");
}

// Function to update/edit a todo's title
// We take a mutable reference to the vector so we can modify it
fn edit_todo(todos: &mut Vec<Todo>, id: u32, new_title: String) {
    println!("\n Editing todo with ID: {} to new title: '{}'", id, new_title);
    
    // iter_mut() gives us mutable references to each element
    // This allows us to modify the todos in place
    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.title = new_title;
            println!(" Todo updated successfully!");
            return; // Exit once we find and update the todo
        }
    }
    
    // If we reach here, the todo wasn't found
    println!(" Error: Todo with ID {} not found!", id);
}

// Function to mark a todo as completed
fn mark_completed(todos: &mut Vec<Todo>, id: u32) {
    println!("\n Marking todo with ID: {} as completed", id);
    
    // Similar pattern to edit_todo but we're changing the completed status
    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.completed = true;
            println!(" Todo marked as completed!");
            return;
        }
    }
    
    println!(" Error: Todo with ID {} not found!", id);
}

// Function to delete a todo from the list
fn delete_todo(todos: &mut Vec<Todo>, id: u32) {
    println!("\n Deleting todo with ID: {}", id);
    
    // Get the original length to check if deletion happened
    let original_len = todos.len();
    
    // retain() keeps only the elements that match the condition
    // We keep todos that DON'T have the specified ID
    todos.retain(|todo| todo.id != id);
    
    // Check if the length changed (meaning we deleted something)
    if todos.len() < original_len {
        println!(" Todo deleted successfully!");
    } else {
        println!(" Error: Todo with ID {} not found!", id);
    }
}

// Function to display all todos in a nice format
fn display_todos(todos: &Vec<Todo>) {
    println!("\n Current Todo List:");
    println!("{}", "=".repeat(50)); // Create a separator line
    
    if todos.is_empty() {
        println!(" No todos found. Your list is empty!");
    } else {
        // Enumerate gives us both the index and the item
        for (index, todo) in todos.iter().enumerate() {
            println!("{}. {}", index + 1, todo);
        }
    }
    
    println!("{}", "=".repeat(50));
}

// Main function - the entry point of our program
fn main() {
    println!(" Welcome to the Rust Todo List Manager!");
    println!("This program demonstrates all CRUD operations on a todo list.\n");
    
    // Create an empty vector to store our todos
    // Vec::new() creates a new, empty vector
    // mut makes it mutable so we can modify it
    let mut todos: Vec<Todo> = Vec::new();
    
    println!(" Starting with empty todo list:");
    display_todos(&todos);
    
    // Demonstrate CREATE operation
    println!("\n{}", "=".repeat(60));
    println!("🔨 DEMONSTRATING CREATE OPERATION");
    println!("{}", "=".repeat(60));
    
    create_todo(&mut todos, 1, "Learn Rust programming".to_string());
    create_todo(&mut todos, 2, "Build a todo app".to_string());
    create_todo(&mut todos, 3, "Master Rust ownership".to_string());
    display_todos(&todos);
    
    // Try to create a duplicate (should fail)
    create_todo(&mut todos, 1, "Duplicate todo".to_string());
    
    // Demonstrate EDIT operation
    println!("\n{}", "=".repeat(60));
    println!(" DEMONSTRATING EDIT OPERATION");
    println!("{}", "=".repeat(60));
    
    edit_todo(&mut todos, 1, "Master Rust programming fundamentals".to_string());
    display_todos(&todos);
    
    // Try to edit non-existent todo
    edit_todo(&mut todos, 99, "This won't work".to_string());
    
    // Demonstrate MARK AS COMPLETED operation
    println!("\n{}", "=".repeat(60));
    println!(" DEMONSTRATING MARK AS COMPLETED OPERATION");
    println!("{}", "=".repeat(60));
    
    mark_completed(&mut todos, 2);
    mark_completed(&mut todos, 3);
    display_todos(&todos);
    
    // Try to mark non-existent todo as completed
    mark_completed(&mut todos, 99);
    
    // Demonstrate DELETE operation
    println!("\n{}", "=".repeat(60));
    println!(" DEMONSTRATING DELETE OPERATION");
    println!("{}", "=".repeat(60));
    
    delete_todo(&mut todos, 2);
    display_todos(&todos);
    
    // Try to delete non-existent todo
    delete_todo(&mut todos, 99);
    
    // Final state
    println!("\n{}", "=".repeat(60));
    println!(" FINAL TODO LIST STATE");
    println!("{}", "=".repeat(60));
    display_todos(&todos);
    
    println!("\n Todo List Manager demonstration completed!");
    println!("All CRUD operations (Create, Read, Update, Delete) have been tested.");
}
