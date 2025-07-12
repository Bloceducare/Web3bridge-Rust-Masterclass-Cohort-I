use std::io::{self, Write};

struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    fn create(id: u32, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    fn update(&mut self, new_title: String) {
        self.title = new_title;
    }

    fn delete(&mut self) {
        self.title = String::new();
        self.completed = false;
    }

    fn edit(&mut self, new_title: String) {
        self.title = new_title;
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();
    println!("Welcome to the Todo List Application!");
    
    loop {
        if todo_list.len() == 0 {
            println!("You have no Todo in your list of Todo");
        } else {
            println!("\nYour Current Todo List:");
            println!("******************************************");
            for todo in &todo_list {
                println!("==> ID: {}, Title: {}, Completed: {}", todo.id, todo.title, todo.completed);
            }
            println!("******************************************");
            println!();
        }
        let mut user_input = String::from("");
        println!("Kindly choose an Option you wish to perform:");
        println!("1. Create Todo");
        println!("2. Update Todo");
        println!("3. Delete Todo");
        println!("4. Edit Todo");
        println!("5. Mark Todo as Completed");
        println!("6. Exit");
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let choice: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                println!("Your Input is => {}", user_input.trim());
                continue
            }
        };

        match choice {
            1 => {
                print!("Enter Todo title: ");
                io::stdout().flush().unwrap();

                user_input.clear();
                io::stdin().read_line(&mut user_input).expect("Failed to read line");
                if user_input.trim().is_empty() {
                    println!("**OUTPUT** Todo title cannot be empty.");
                    continue;
                }
                let todo = Todo::create(todo_list.len() as u32 + 1, user_input.trim().to_string());
                todo_list.push(todo);
                println!("**OUTPUT** Todo created successfully!");
            }
            2 => {
                print!("Enter Todo ID to update: ");
                io::stdout().flush().unwrap();
                user_input.clear();
                io::stdin().read_line(&mut user_input).expect("Failed to read line");
                let id: u32 = user_input.trim().parse().expect("Please enter a valid number");
                if let Some(todo) = todo_list.iter_mut().find(|todo| todo.id == id) {
                    print!("Enter new title: ");
                    io::stdout().flush().unwrap();
                    user_input.clear();
                    io::stdin().read_line(&mut user_input).expect("Failed to read line");
                    todo.update(user_input.trim().to_string());
                    println!("**OUTPUT** Todo updated successfully!");
                } else {
                    println!("**OUTPUT** Todo with ID {} not found.", id);
                }
            }
            3 => {
                print!("Enter Todo ID to delete: ");
                io::stdout().flush().unwrap();
                user_input.clear();
                io::stdin().read_line(&mut user_input).expect("Failed to read line");
                let id: u32 = user_input.trim().parse().expect("Please enter a valid number");
                if let Some(todo) = todo_list.iter_mut().find(|todo| todo.id == id) {
                    todo.delete();
                    todo_list.remove((id - 1).try_into().unwrap());
                    println!("**OUTPUT** Todo deleted successfully!");
                } else {
                    println!("**OUTPUT** Todo with ID {} not found.", id);
                }
            }
            4 => {
                print!("Enter Todo ID to edit: ");
                io::stdout().flush().unwrap();
                user_input.clear();
                io::stdin().read_line(&mut user_input).expect("Failed to read line");
                let id: u32 = user_input.trim().parse().expect("Please enter a valid number");
                if let Some(todo) = todo_list.iter_mut().find(|todo| todo.id == id) {
                    print!("Enter new title: ");
                    io::stdout().flush().unwrap();
                    user_input.clear();
                    io::stdin().read_line(&mut user_input).expect("Failed to read line");
                    todo.edit(user_input.trim().to_string());
                    println!("**OUTPUT** Todo edited successfully!");
                } else {
                    println!("**OUTPUT** Todo with ID {} not found.", id);
                }
            }
            5 => {
                print!("Enter Todo ID to mark as completed: ");
                io::stdout().flush().unwrap();
                user_input.clear();
                io::stdin().read_line(&mut user_input).expect("Failed to read line");
                let id: u32 = user_input.trim().parse().expect("Please enter a valid number");
                if let Some(todo) = todo_list.iter_mut().find(|todo| todo.id == id) {
                    todo.mark_completed();
                    println!("**OUTPUT** Todo marked as completed successfully!");
                } else {
                    println!("**OUTPUT** Todo with ID {} not found.", id);
                }
            }
            6 => {
                println!("Exiting the Todo List Application. Goodbye!");
                break; 
            }
            _ => {
                println!("Invalid choice. Please try again.");
                continue
            }
        }
    };
}
