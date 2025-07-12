enum TodoStatus {
    Created,
    Edited,
    Completed,
}

struct Todo {
    id: u32,
    title: String,
    description: String,
    status: TodoStatus,
}

struct TodoList {
    todos: Vec<Todo>,
    current_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        println!("############ - Todo List Instantiated - ####################");
        TodoList {
            todos: Vec::new(),
            current_id: 1,
        }
    }

    fn create(&mut self, title: String, description: String) {
        let todo = Todo {
            id: self.current_id,
            title,
            description,
            status: TodoStatus::Created,
        };

        self.todos.push(todo);
        self.current_id += 1;

        println!("#{} Todo created", self.current_id - 1);
        println!("-----------------------------------");
    }

    fn read(&self) {
        if self.todos.is_empty() {
            println!("No todos found!");
        } else {
            for todo in &self.todos {
                let formatted_status = match todo.status {
                    TodoStatus::Created => "Created",
                    TodoStatus::Edited => "Edited",
                    TodoStatus::Completed => "Completed",
                };

                println!("#{} Todo", todo.id);
                println!(
                    "Title: {}, Description: {}, Status: {}",
                    todo.title, todo.description, formatted_status
                );
                println!("-----------------------------------");
            }
        }
    }

    fn update(&mut self, id: u32, title: String, description: String) {
        match self.todos.iter_mut().find(|todo| todo.id == id) {
            Some(todo) => {
                todo.title = title;
                todo.description = description;
                todo.status = TodoStatus::Edited;

                println!("#{} Todo Updated", id);
                println!("-----------------------------------");
            }
            None => {
                println!("Todo with Id {} not found", id);
            }
        }
    }

    fn delete(&mut self, id: u32) {
        match self.todos.iter().position(|todo| todo.id == id) {
            Some(index) => {
                self.todos.remove(index);

                println!("#{} Todo Deleted", id);
                println!("-----------------------------------");
            }
            None => {
                println!("Todo with Id {} not found", id);
            }
        }
    }

    fn mark_completed(&mut self, id: u32) {
        match self.todos.iter_mut().find(|todo| todo.id == id) {
            Some(todo) => {
                todo.status = TodoStatus::Completed;

                println!("#{} Todo marked as Completed", id);
                println!("-----------------------------------");
            }
            None => {
                println!("Todo with Id {} not found", id);
            }
        }
    }

    fn get_todo(&self, id: u32) {
        match self.todos.iter().find(|todo| todo.id == id) {
            Some(todo) => {
                let formatted_status = match todo.status {
                    TodoStatus::Created => "Created",
                    TodoStatus::Edited => "Edited",
                    TodoStatus::Completed => "Completed",
                };
                println!("#{} Todo", todo.id);
                println!(
                    "Title: {}, Description: {}, Status: {}",
                    todo.title, todo.description, formatted_status
                );
                println!("-----------------------------------");
            }
            None => {
                println!("Todo with Id {} not found", id);
            }
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.create(
        "Learn Rust".to_string(),
        "Complete the Rust course".to_string(),
    );
    todo_list.create(
        "Build a project".to_string(),
        "Create a simple Rust application".to_string(),
    );

    todo_list.read();

    todo_list.update(
        1,
        "Learn Rust Basics".to_string(),
        "Complete the basics of Rust".to_string(),
    );
    todo_list.mark_completed(2);

    todo_list.read();

    todo_list.get_todo(1);
    todo_list.get_todo(2);

    todo_list.delete(1);
    todo_list.read();

    todo_list.delete(3); // Attempt to delete a non-existent todo
}
