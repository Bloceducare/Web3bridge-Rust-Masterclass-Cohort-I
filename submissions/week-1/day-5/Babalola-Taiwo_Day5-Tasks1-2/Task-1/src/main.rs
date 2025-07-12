use std::fmt;

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();

    create_todo(&mut todo_list, "Buy groceries", 1);
    create_todo(&mut todo_list, "Clean the room", 2);
    create_todo(&mut todo_list, "Learn Rust", 3);

    show_all_tasks(&todo_list);

     println!(" ");

    update_todo(&mut todo_list, 2, "Clean and organize the room");

    mark_todo_completed(&mut todo_list, 3);

    delete_todo(&mut todo_list, 1);

    // show_all_tasks(&todo_list);
}

#[derive(Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.completed { "✅ Done" } else { "Pending...." };
        write!(f, "Todo #{} - {} [{}]", self.id, self.title, status)
    }
}

fn create_todo(todos: &mut Vec<Todo>, title: &str, id: u32) {
    let new_task = Todo {
        id,
        title: title.to_string(),
        completed: false,
    };
    todos.push(new_task);
    println!("✅ Added: {}", title);
}

fn update_todo(todos: &mut Vec<Todo>, id: u32, new_title: &str) {
    for task in todos.iter_mut() {
        if task.id == id {
            task.title = new_title.to_string();
            println!("✍🏾  Edited Todo #{} to '{}'", id, new_title);
            return;
        }
    }
    println!("❌ Todo with ID {} not found.", id);
}

fn mark_todo_completed(todos: &mut Vec<Todo>, id: u32) {
    for task in todos.iter_mut() {
        if task.id == id {
            task.completed = true;
            println!("✅  Marked Todo #{} as completed", id);
            return;
        }
    }
    println!("❌ Todo with ID {} not found.", id);
}

fn delete_todo(todos: &mut Vec<Todo>, id: u32) {
    let original_len = todos.len();
    todos.retain(|task| task.id != id);

    if todos.len() < original_len {
        println!("🗑️  Deleted Todo #{}", id);
    } else {
        println!("❌ Todo with ID {} not found.", id);
    }
}

fn show_all_tasks(todos: &Vec<Todo>) {
    println!("\n📋 Current Todo List:");
    for task in todos {
        println!("  → {}", task);
    }
}
