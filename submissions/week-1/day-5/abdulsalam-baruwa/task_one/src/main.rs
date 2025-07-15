use std::collections::HashMap;

struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

fn create_todo(
    todos: &mut HashMap<u32, Todo>,
    next_id: &mut u32,
    title: String,
    description: String,
) -> u32 {
    // Get the current ID and increment it
    let id = *next_id;
    *next_id += 1;

    // Create the new todo
    let todo = Todo {
        id,
        title,
        description,
        completed: false,
    };

    println!(
        "\nTodo Created!\nid: {}\ntitle: {}\ndescription: {}\ncompleted: {}",
        id, todo.title, todo.description, todo.completed
    );

    // Add it to our HashMap
    todos.insert(id, todo);
    id
}

fn update_todo(
    todos: &mut HashMap<u32, Todo>,
    id: u32,
    title: String,
    description: String,
    completed: bool,
) -> bool {
    if let Some(todo) = todos.get_mut(&id) {
        todo.title = title;
        todo.description = description;
        todo.completed = completed;
        println!("Updated todo with ID: {}", id);
        true
    } else {
        println!("Todo with ID {} not found", id);
        false
    }
}

fn delete_todo(todos: &mut HashMap<u32, Todo>, id: u32) -> bool {
    if let Some(todo) = todos.remove(&id) {
        println!("Deleted todo with ID: {} (Title: '{}')", id, todo.title);
        true
    } else {
        println!("Todo with ID {} not found", id);
        false
    }
}

fn edit_todo(
    todos: &mut HashMap<u32, Todo>,
    id: u32,
    title: Option<String>,
    description: Option<String>,
) -> bool {
    if let Some(todo) = todos.get_mut(&id) {
        if let Some(new_title) = title {
            todo.title = new_title;
            println!("Updated title for todo ID: {}", id);
        }
        if let Some(new_description) = description {
            todo.description = new_description;
            println!("Updated description for todo ID: {}", id);
        }
        true
    } else {
        println!("Todo with ID {} not found", id);
        false
    }
}

fn mark_completed(todos: &mut HashMap<u32, Todo>, id: u32) -> bool {
    if let Some(todo) = todos.get_mut(&id) {
        todo.completed = true;
        println!("Marked todo with ID: {} as completed", id);
        true
    } else {
        println!("Todo with ID {} not found", id);
        false
    }
}

fn main() {
    // Create our todo storage
    let mut todos: HashMap<u32, Todo> = HashMap::new();
    let mut next_id: u32 = 1;

    println!("=== Testing Todo List Functions ===\n");

    // Test 1: Create todos
    println!("1. Creating todos:");
    let todo1_id = create_todo(
        &mut todos,
        &mut next_id,
        String::from("Learn Rust"),
        String::from("Complete the todo list project to understand Rust basics"),
    );

    let todo2_id = create_todo(
        &mut todos,
        &mut next_id,
        String::from("Practice coding"),
        String::from("Write more Rust programs to improve skills"),
    );

    println!("\n2. Updating a todo (replacing entire content):");
    update_todo(
        &mut todos,
        todo1_id,
        String::from("Master Rust"),
        String::from("Become proficient in Rust programming language"),
        true,
    );

    println!("\n3. Editing specific fields of a todo:");
    edit_todo(
        &mut todos,
        todo2_id,
        Some(String::from("Practice Rust coding")),
        None, // Don't change description
    );

    println!("\n4. Marking a todo as completed:");
    mark_completed(&mut todos, todo1_id);

    println!("\n5. Deleting a todo:");
    delete_todo(&mut todos, todo2_id);

    println!("\n6. Trying to access a deleted todo:");
    mark_completed(&mut todos, todo2_id); // This should fail

    println!("\n=== Final todo count: {} ===", todos.len());
}