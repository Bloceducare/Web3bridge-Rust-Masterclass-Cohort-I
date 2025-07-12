use std::io;

#[derive(Debug)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

fn complete_item(item: &mut TodoItem) {
    item.completed = true;
    println!("Item '{}' has been marked as completed!", item.title);
}

fn display_items(todo_list: &Vec<TodoItem>) {
    if todo_list.is_empty() {
        println!("No to-do items found.");
        return;
    }
    
    println!("\n--- Todo List ---");
    for item in todo_list {
        let status = if item.completed { "✓" } else { "✗" };
        println!("ID: {}, Title: {}, Completed: {} {}", 
                 item.id, item.title, item.completed, status);
    }
    println!("----------------\n");
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        println!("What would you like to do?");
        println!("1. Add a to-do item");
        println!("2. Complete a to-do item");
        println!("3. Display to-do items");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = match choice.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter the name of the to-do item:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                if name.is_empty() {
                    println!("Todo item name cannot be empty!");
                    continue;
                }

                let id = todo_list.len() as u32 + 1;

                let item = TodoItem {
                    id,
                    title: name,
                    completed: false,
                };

                todo_list.push(item);
                println!("Todo item added successfully!");
            },
            2 => {
                if todo_list.is_empty() {
                    println!("No to-do items to complete!");
                    continue;
                }

                display_items(&todo_list);
                println!("Enter the ID of the to-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                
                let id = match id.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                match todo_list.iter_mut().find(|i| i.id == id) {
                    Some(item) => {
                        if item.completed {
                            println!("Item '{}' is already completed!", item.title);
                        } else {
                            complete_item(item);
                        }
                    },
                    None => {
                        println!("No to-do item found with ID: {}", id);
                    }
                }
            },
            3 => {
                display_items(&todo_list);
            },
            4 => {
                println!("Goodbye!");
                return;
            },
            _ => {
                println!("Invalid choice. Please select 1-4.");
            },
        }
    }
}