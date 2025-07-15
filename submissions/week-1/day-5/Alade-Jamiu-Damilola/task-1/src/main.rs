#![allow(unused)]
use std::io;

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

fn create_todo(todos: &mut Vec<Todo>, title: &str, description: &str) -> u32 {
    let id = (todos.len() as u32) + 1; 
    let todo = Todo {
        id,
        title: title.trim().to_string(),
        description: description.trim().to_string(),
        completed: false,
    };
    todos.push(todo);
    println!("Created Todo: ID={}, Title={}, Description={}, Completed={}",
             id, title, description, false);
    id
}

fn update_todo(todos: &mut Vec<Todo>, id: u32, new_title: &str, new_description: &str) -> bool {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.title = new_title.trim().to_string();
        todo.description = new_description.trim().to_string();
        println!("Updated Todo: ID={}, Title={}, Description={}, Completed={}",
                 id, todo.title, todo.description, todo.completed);
        true
    } else {
        println!("Todo with ID={} not found", id);
        false
    }
}

fn delete_todo(todos: &mut Vec<Todo>, id: u32) -> bool {
    if let Some(index) = todos.iter().position(|t| t.id == id) {
        let todo = todos.remove(index);
        println!("Deleted Todo: ID={}, Title={}, Description={}, Completed={}",
                 todo.id, todo.title, todo.description, todo.completed);
        true
    } else {
        println!("Todo with ID={} not found", id);
        false
    }
}

fn edit_todo(todos: &mut Vec<Todo>, id: u32, new_title: &str, new_description: &str) -> bool {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.title = new_title.trim().to_string();
        todo.description = new_description.trim().to_string();
        println!("Edited Todo: ID={}, Title={}, Description={}, Completed={}",
                 id, todo.title, todo.description, todo.completed);
        true
    } else {
        println!("Todo with ID={} not found", id);
        false
    }
}

fn mark_todo_completed(todos: &mut Vec<Todo>, id: u32) -> bool {
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        todo.completed = true;
        println!("Marked Todo as completed: ID={}, Title={}, Description={}, Completed={}",
                 id, todo.title, todo.description, todo.completed);
        true
    } else {
        println!("Todo with ID={} not found", id);
        false
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_id(prompt: &str) -> u32 {
    loop {
        let input = read_input(prompt);
        match input.parse::<u32>() {
            Ok(id) => return id,
            Err(_) => println!("Please enter a valid number for ID"),
        }
    }
}

fn display_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("No todos found.");
    } else {
        println!("\nCurrent Todo List:");
        for todo in todos {
            println!("{:?}", todo);
        }
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        println!("\nTodo List Menu:");
        println!("1. Create Todo");
        println!("2. Update Todo");
        println!("3. Edit Todo");
        println!("4. Mark Todo as Completed");
        println!("5. Delete Todo");
        println!("6. Display Todos");
        println!("7. Exit");

        let choice = read_input("Enter your choice (1-7):");

        match choice.as_str() {
            "1" => {
                let title = read_input("Enter todo title:");
                let description = read_input("Enter todo description:");
                create_todo(&mut todos, &title, &description);
            }
            "2" => {
                let id = read_id("Enter todo ID to update:");
                let new_title = read_input("Enter new title:");
                let new_description = read_input("Enter new description:");
                update_todo(&mut todos, id, &new_title, &new_description);
            }
            "3" => {
                let id = read_id("Enter todo ID to edit:");
                let new_title = read_input("Enter new title:");
                let new_description = read_input("Enter new description:");
                edit_todo(&mut todos, id, &new_title, &new_description);
            }
            "4" => {
                let id = read_id("Enter todo ID to mark as completed:");
                mark_todo_completed(&mut todos, id);
            }
            "5" => {
                let id = read_id("Enter todo ID to delete:");
                delete_todo(&mut todos, id);
            }
            "6" => {
                display_todos(&todos);
            }
            "7" => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice, please select 1-7"),
        }
    }
}