use std::io;

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn create_todo(id: u32, title: &str) -> Todo {
    Todo {
        id,
        title: title.to_string(),
        completed: false,
    }
}

fn update_todo(todo: &mut Todo, new_title: &str) {
    todo.title = new_title.to_string();
}

fn delete_todo(todos: &mut Vec<Todo>, id: u32) {
    todos.retain(|todo| todo.id != id);
}

fn mark_completed(todo: &mut Todo) {
    todo.completed = true;
}

fn edit_todo(
    todos: &mut Vec<Todo>,
    id: u32,
    new_title: &str,
    completed: bool,
) -> Result<(), String> {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.title = new_title.to_string();
        todo.completed = completed;
        Ok(())
    } else {
        Err(format!("Todo with ID {} not found.", id))
    }
}

fn display_todos(todos: &[Todo]) {
    println!("\n--- TODO LIST ---");
    for todo in todos {
        println!(
            "[{}] {} - {}",
            todo.id,
            if todo.completed { "✓" } else { " " },
            todo.title
        );
    }
    println!("-----------------\n");
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut next_id: u32 = 1;

    loop {
        println!(
            "What would you like to do? (add / update / complete / edit / delete / display / quit)"
        );

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim().to_lowercase();

        match command.as_str() {
            "add" => {
                println!("Enter title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed");
                let todo = create_todo(next_id, title.trim());
                todos.push(todo);
                println!("Todo added!");
                next_id += 1;
            }

            "update" => {
                println!("Enter ID to update:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed");
                let id = id.trim().parse::<u32>().unwrap_or(0);

                if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                    println!("Enter new title:");
                    let mut title = String::new();
                    io::stdin().read_line(&mut title).expect("Failed");
                    update_todo(todo, title.trim());
                    println!("Todo updated!");
                } else {
                    println!("Todo not found.");
                }
            }

            "complete" => {
                println!("Enter ID to mark as completed:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed");
                let id = id.trim().parse::<u32>().unwrap_or(0);

                if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                    mark_completed(todo);
                    println!("Todo marked as completed!");
                } else {
                    println!("Todo not found.");
                }
            }

            "edit" => {
                println!("Enter ID to edit:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed");
                let id = id.trim().parse::<u32>().unwrap_or(0);

                println!("Enter new title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed");

                println!("Is it completed? (yes/no):");
                let mut completed = String::new();
                io::stdin().read_line(&mut completed).expect("Failed");
                let status = completed.trim().eq_ignore_ascii_case("yes");

                match edit_todo(&mut todos, id, title.trim(), status) {
                    Ok(_) => println!("Todo edited!"),
                    Err(e) => println!("Error: {}", e),
                }
            }

            "delete" => {
                println!("Enter ID to delete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed");
                let id = id.trim().parse::<u32>().unwrap_or(0);

                delete_todo(&mut todos, id);
                println!("Todo deleted!");
            }

            "display" => {
                display_todos(&todos);
            }

            "quit" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!(
                    "Invalid command. Try one of: add, update, complete, edit, delete, display, quit"
                );
            }
        }
    }
}
