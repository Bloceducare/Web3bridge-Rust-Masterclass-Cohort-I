use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Todo {
    id: usize,
    title: String,
    description: String,
    updated_at: DateTime<Local>,
    completed: bool,
}

pub struct Todos {
    pub todos: Vec<Todo>,
}

impl Todos {
    fn new() -> Self {
        Todos { todos: Vec::new() }
    }

    fn check_id(&self, id: usize) {
        if id > self.todos.len() || id < 1 {
            panic!("Todo with ID {id} does not exist");
        }
    }

    fn create_todo(&mut self, title: String, description: String) {
        let id = self.todos.len() + 1;

        let todo = Todo {
            id,
            title,
            description,
            updated_at: Local::now(),
            completed: false,
        };

        self.todos.push(todo);

        println!("Todo {id} created successfully");
    }

    fn update_todo(&mut self, id: usize, new_title: String, new_description: String) {
        self.check_id(id);

        for todo in &mut self.todos {
            if todo.id == id {
                todo.title = new_title;
                todo.description = new_description;
                todo.updated_at = Local::now();

                println!("Updated todo ID: {id}");
                return; //this will stop it from running after the ID is found
            }
        }
        println!("Todo with ID {id} not found");
    }

    fn delete_todo(&mut self, id: usize) {
        self.check_id(id);

        for todo in &mut self.todos {
            if todo.id == id {
                let deleted_todo = self.todos.remove(id - 1);

                println!(" Todo with ID {id} deleted");
                return;
            }
        }
        println!("Todo with ID {id} not found");
    }

    fn mark_completed(&mut self, id: usize) {
        self.check_id(id);

        for todo in &mut self.todos {
            if todo.id == id {
                todo.completed = true;
                todo.updated_at = Local::now();

                println!("Todo with ID: {id} is completed");
                return;
            }
        }
        println!("Todo with ID {id} not found");
    }

    fn completed_todos(&self) {
        if self.todos.is_empty() {
            println!("Empty Todo List");
            return;
        }

        for todo in &self.todos {
            if todo.completed {
                println!("Completed {:#?}", { todo });
            }
        }
    }

    fn incompleted_todos(&self) {
        if self.todos.is_empty() {
            println!("Empty Todo List");
            return;
        }

        for todo in &self.todos {
            if !todo.completed {
                println!("Pending {:#?}", { todo });
            }
        }
    }

    fn all_todos(&self) {
        if self.todos.is_empty() {
            println!("Empty Todo List");
            return;
        }

        println!("All {:#?}", self.todos);
    }
}

fn main() {
    let mut todo_app = Todos::new();

    println!("Creating todos...");

    todo_app.create_todo(
        String::from("Make lots of money"),
        String::from("I want to make lots of money before Q4"),
    );

    todo_app.create_todo(
        String::from("Vacation"),
        String::from("I want to go for, at least, 3 vacations before the end of the year"),
    );

    todo_app.create_todo(
        String::from("Go Rusty"),
        String::from("Before the end Q3, I want to learn Rust on protocol level"),
    );

    todo_app.all_todos();

    println!("Updating a todos...");
    todo_app.update_todo(
        1,
        String::from("Make lots of money"),
        String::from("I have made some but I need to make more."),
    );

    println!("Completed todos...");
    todo_app.mark_completed(3);

    todo_app.completed_todos();

    println!("Show the incompleted todos...");
    todo_app.incompleted_todos();

    println!("Deleting a todo...");
    todo_app.delete_todo(3);

    todo_app.all_todos();
}