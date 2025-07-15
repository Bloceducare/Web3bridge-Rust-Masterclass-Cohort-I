#[derive(Debug)]
enum ToDoStatus {
    PENDING,
    COMPLETED,
}

struct ToDo {
    title: String,
    description: String,
    status: ToDoStatus,
}

impl ToDo {
    fn create_todo(title: String, description: String) -> ToDo {
        let todo = ToDo {
            title: title,
            description: description,
            status: ToDoStatus::PENDING,
        };
        println!("Todo Created: {}", todo.title);
        todo
    }

    fn update_todo(&mut self, new_title: String, new_description: String) {
        self.title = new_title;
        self.description = new_description;
        println!("Todo Updated: {}", self.title);
    }

    fn edit_todo(&mut self, new_title: String, new_description: String) {
        self.title = new_title;
        self.description = new_description;
        println!("Todo Edited: {}", self.title);
    }

    fn mark_completed(&mut self) {
        self.status = ToDoStatus::COMPLETED;
        println!("Todo Marked as Completed: {}", self.title);
    }

    fn display(&self) {
        println!(
            "Title: {}, Description: {}, Status: {:?}",
            self.title, self.description, self.status
        );
    }
}

fn delete_todo(todos: &mut Vec<ToDo>, index: usize) {
    if index < todos.len() {
        let removed_todo = todos.remove(index);
        println!("Todo Deleted: {}", removed_todo.title);
    } else {
        println!("Invalid index for deletion");
    }
}

fn main() {
    println!("=== Todo List Management System ===");

    // Create todos
    let mut todo1 = ToDo::create_todo(
        String::from("Learn Rust"),
        String::from("Learning at web3bridge"),
    );

    let mut todo2 = ToDo::create_todo(
        String::from("Build Project"),
        String::from("Create a web3 application"),
    );

    // Display todos
    println!("\n--- Initial Todos ---");
    todo1.display();
    todo2.display();

    // Update todo
    println!("\n--- Updating Todo ---");
    todo1.update_todo(
        String::from("Master Rust"),
        String::from("Complete Rust masterclass"),
    );

    // Edit todo
    println!("\n--- Editing Todo ---");
    todo2.edit_todo(
        String::from("Deploy Project"),
        String::from("Deploy the web3 application to production"),
    );

    // Mark as completed
    println!("\n--- Marking as Completed ---");
    todo1.mark_completed();

    // Display final state
    println!("\n--- Final State ---");
    todo1.display();
    todo2.display();

    // Demonstrate delete functionality with a vector
    println!("\n--- Delete Demo ---");
    let mut todos = vec![todo1, todo2];
    delete_todo(&mut todos, 0);
    println!("Remaining todos: {}", todos.len());
}

cargo run 
   Compiling task-1 v0.1.0 (/Users/amityclev/Documents/dev/rustweek/week1/Web3bridge-Rust-Masterclass-Cohort-I/submissions/week-1/day-5/Bolarinwa-Muhdsodiq/task-1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 36.45s
     Running `target/debug/task-1`
=== Todo List Management System ===
Todo Created: Learn Rust
Todo Created: Build Project

--- Initial Todos ---
Title: Learn Rust, Description: Learning at web3bridge, Status: PENDING
Title: Build Project, Description: Create a web3 application, Status: PENDING

--- Updating Todo ---
Todo Updated: Master Rust

--- Editing Todo ---
Todo Edited: Deploy Project

--- Marking as Completed ---
Todo Marked as Completed: Master Rust

--- Final State ---
Title: Master Rust, Description: Complete Rust masterclass, Status: COMPLETED
Title: Deploy Project, Description: Deploy the web3 application to production, Status: PENDING

--- Delete Demo ---
Todo Deleted: Master Rust
Remaining todos: 1
