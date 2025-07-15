// # Task 1: Todo list in Rust

// ## Objective

// Write a Rust program with separate functions to:

// - Create Todo
// - Update Todo
// - Delete Todo
// - Edit Todo
// - Mark todo as completed
// - Then you should attach a screenshot of the printed version, just one image containing the result of the logs of that function.
// - Attach the screenshot to the code and Push your code to github also, don't submit screenshot alone.

// Then call each of them in `main()` with values and run the program using `cargo run`.
// get the task from user input trough the terminal

use std::io;
use std::io::Write;
#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn main (){
    let mut todos: Vec<Todo> = Vec::new();
    let mut id_counter = 1;

    loop {
        println!("Choose an option:");
        println!("1. Create Todo");
        println!("2. View Todo");
        println!("3. Update Todo");
        println!("4. Delete Todo");
        println!("5. Edit Todo");
        println!("6. Mark Todo as Completed");
        println!("7. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let mut title = String::new();
                print!("Enter todo title: ");
                io::stdout().flush().unwrap(); 
                io::stdin().read_line(&mut title).expect("Failed to read line");
                let todo = Todo {
                    id: id_counter,
                    title: title.trim().to_string(),
                    completed: false,
                };
                todos.push(todo);
                id_counter += 1;
                println!("Todo created successfully!");
            }
            2 => {
                println!("Todo List:");
                for todo in &todos {
                    println!("ID: {}, Title: {}, Completed: {}", todo.id, todo.title, todo.completed);
                }
            }
            3 => {
                let mut id_input = String::new();
                print!("Enter todo ID to update: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_input).expect("Failed to read line");
                let id: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                
                if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                    let mut new_title = String::new();
                    print!("Enter new title for todo {}: ", id);
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut new_title).expect("Failed to read line");
                    todo.title = new_title.trim().to_string();
                    println!("Todo updated successfully!");
                } else {
                    println!("Todo with ID {} not found.", id);
                }
            }
            4 => {
                let mut id_input = String::new();
                print!("Enter todo ID to delete: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_input).expect("Failed to read line");
                let id: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                todos.retain(|t| t.id != id);
                println!("Todo deleted successfully!");
            }
            5 => {
                println!("Edit the title of the todo.");
                let mut id_input = String::new();
                print!("Enter todo ID to edit: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_input).expect("Failed to read line");
                let id: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                    let mut new_title = String::new();
                    print!("Enter new title for todo {}: ", id);
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut new_title).expect("Failed to read line");
                    todo.title = new_title.trim().to_string();
                    println!("Todo updated successfully!");
                } else {
                    println!("Todo with ID {} not found.", id);
                }
            }
            6=> {
                println!("Mark todo as completed.");

                let mut id_input = String::new();
                print!("Enter todo ID to mark as completed: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_input).expect("Failed to read line");
                let id: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                    todo.completed = true;
                    println!("Todo marked as completed!");
                } else {
                    println!("Todo with ID {} not found.", id);
                }
            }
            7 => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}