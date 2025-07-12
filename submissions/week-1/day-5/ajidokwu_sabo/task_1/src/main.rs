#[derive(Debug, Clone)]
pub struct Todo {
    id: u8,
    title: String,
    description: String,
    status: bool, // false = not completed, true = completed
}

// Create a new todo item
fn create_todo(id: u8, title: String, description: String) -> Todo {
    Todo {
        id,
        title,
        description,
        status: false,
    }
}

// Update the title and description of a todo
fn update_todo(todo: &mut Todo, title: String, description: String) {
    todo.title = title;
    todo.description = description;
    println!("Updated Todo: {:?}", todo);
}

// Edit only the title
fn edit_todo(todo: &mut Todo, title: String) {
    todo.title = title;
    println!("Edited Todo Title: {:?}", todo);
}

// Mark todo as completed
fn mark_todo_completed(todo: &mut Todo) {
    todo.status = true;
    println!("Marked as completed: {:?}", todo);
}

// Delete a todo (for this example, we just print since Rust doesn't have GC delete)
// Usually you'd remove it from a Vec
fn delete_todo(todo: Todo) {
    println!("Deleting Todo: {:?}", todo);
    // drop(todo); // optional
}

fn main() {
    // Create a todo
    let mut my_todo = create_todo(1, "Learn Rust".to_string(), "Complete Rust basics module".to_string());
    println!("Created Todo: {:?}", my_todo);

    // Update the todo
    update_todo(&mut my_todo, "Learn Rust Advanced".to_string(), "Move to ownership concepts".to_string());

    // Edit just the title
    edit_todo(&mut my_todo, "Master Rust".to_string());

    // Mark it as completed
    mark_todo_completed(&mut my_todo);

    // Delete the todo
    delete_todo(my_todo);
}
