use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush stdout to display prompt
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn create_todo(todos: &mut Vec<Todo>, id_counter: &mut u32) {
    loop {
        let title = get_input("Enter task title: ");
        let todo = Todo {
            id: *id_counter,
            title,
            completed: false,
        };
        todos.push(todo.clone());
        println!("Created: {:?}", todo);
        *id_counter += 1;

        let next = get_input("Will that be all? (Y/N): ");
        if next.eq_ignore_ascii_case("Y") {
            break;
        }
    }
}

fn edit_todo(todos: &mut Vec<Todo>) {
    loop {
        let id = get_input("Enter ID of todo to edit: ").parse::<u32>().unwrap_or(0);
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            let new_title = get_input("Enter new title: ");
            todo.title = new_title;
            println!("Edited: {:?}", todo);
        } else {
            println!("Todo with ID {} not found.", id);
        }

        let next = get_input("Edit another? (Y/N): ");
        if next.eq_ignore_ascii_case("Y") == false {
            break;
        }
    }
}

fn update_todo(todos: &mut Vec<Todo>) {
    loop {
        let id = get_input("Enter ID of todo to update: ").parse::<u32>().unwrap_or(0);
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            let new_title = get_input("Enter updated title: ");
            todo.title = new_title;
            println!("Updated: {:?}", todo);
        } else {
            println!("Todo with ID {} not found.", id);
        }

        let next = get_input("Update another? (Y/N): ");
        if next.eq_ignore_ascii_case("Y") == false {
            break;
        }
    }
}

fn delete_todo(todos: &mut Vec<Todo>) {
    loop {
        let id = get_input("Enter ID of todo to delete: ").parse::<u32>().unwrap_or(0);
        let len_before = todos.len();
        todos.retain(|t| t.id != id);
        if todos.len() < len_before {
            println!("Todo with ID {} deleted.", id);
        } else {
            println!("Todo with ID {} not found.", id);
        }

        let next = get_input("Delete another? (Y/N): ");
        if next.eq_ignore_ascii_case("Y") == false {
            break;
        }
    }
}

fn mark_completed(todos: &mut Vec<Todo>) {
    loop {
        let id = get_input("Enter ID of todo to mark as completed: ").parse::<u32>().unwrap_or(0);
        if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            println!("Marked as completed: {:?}", todo);
        } else {
            println!("Todo with ID {} not found.", id);
        }

        let next = get_input("Mark another? (Y/N): ");
        if next.eq_ignore_ascii_case("Y") == false {
            break;
        }
    }
}

fn view_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No todos available.");
    } else {
        println!("\n==== TODO LIST ====");
        for todo in todos {
            println!(
                "[ID: {}] {} - {}",
                todo.id,
                todo.title,
                if todo.completed { "✅ Completed" } else { "❌ Not Completed" }
            );
        }
        println!("===================\n");
    }
}

fn main() {
    let mut todos: Vec<Todo> = vec![];
    let mut id_counter: u32 = 1;

    loop {
        println!(
            "\n===== TODO APP MENU =====
1. Create Todo
2. Update Todo
3. Delete Todo
4. Edit Todo
5. Mark Todo as Completed
6. View Todo List
0. Exit"
        );

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => create_todo(&mut todos, &mut id_counter),
            "2" => update_todo(&mut todos),
            "3" => delete_todo(&mut todos),
            "4" => edit_todo(&mut todos),
            "5" => mark_completed(&mut todos),
            "6" => view_todos(&todos),
            "0" => {
                println!("Exiting Todo App. Goodbye!");
                break;
            }
            _ => println!("Invalid choice! Please select a valid option."),
        }
    }
}
