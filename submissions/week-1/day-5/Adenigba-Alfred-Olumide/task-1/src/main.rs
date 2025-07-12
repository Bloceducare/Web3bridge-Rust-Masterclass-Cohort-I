#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl Todo {
    fn new(id: u32, title: String, description: String) -> Self {
        Self { id, title, description, completed: false }
    }
}

#[derive(Debug)]
struct TodoList {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        Self { todos: Vec::new(), next_id: 1 }
    }
}

impl TodoList {
    fn create_todo(&mut self, title: String, description: String) -> u32 {
        let id = self.next_id;
        let todo = Todo::new(id, title, description);
        println!("Todo created with id: {}", id);
        println!("Todo: {:?}", todo);
        println!("--------------------------------");
        self.todos.push(todo);
        self.next_id += 1;
        id
    }

    fn update_todo(&mut self, id: u32, title: String, description: String) -> Result<(), String> {
        let todo = self.todos.iter_mut().find(|todo| todo.id == id).map(|todo| {
            todo.title = title;
            todo.description = description;
        });

        if todo.is_none() {
            let error = format!("Todo with id {} not found", id);
            println!("{}", error);
            return Err(error); 
        }
        println!("Todo with id {} updated.", id);
        println!("Updated Todo: {:?}", self.todos.iter().find(|todo| todo.id == id).unwrap());
        println!("--------------------------------");
        Ok(())
    }

    fn mark_todo_as_completed(&mut self, id: u32) {
        let found = self.todos.iter_mut().find(|todo| todo.id == id).map(|todo| {
            todo.completed = true;
        });
        if found.is_some() {
            println!("Todo with id {} marked as completed.", id);
            println!("Completed Todo: {:?}", self.todos.iter().find(|todo| todo.id == id).unwrap());
        } else {
            println!("Todo with id {} not found", id);
        }
        println!("--------------------------------");
    }

    fn delete_todo(&mut self, id: u32) {
        let len_before = self.todos.len();
        self.todos.retain(|todo| todo.id != id);
        if self.todos.len() < len_before {
            println!("Todo with id {} deleted.", id);
        } else {
            println!("Todo with id {} not found.", id);
        }
        println!("--------------------------------");
    }

    fn get_todo(&self, id: u32) -> Result<&Todo, String> {
        let todo = self.todos.iter().find(|todo| todo.id == id);
        if let Some(todo) = todo {
            println!("Fetched Todo with id {}: {:?}", id, todo);
            println!("--------------------------------");
            Ok(todo)
        } else {
            let error = format!("Todo with id {} not found", id);
            println!("{}", error);
            println!("--------------------------------");
            Err(error)
        }
    }
}


fn main() {
    let mut todo_list = TodoList::new();
    todo_list.create_todo("Buy groceries".to_string(), "Buy groceries from the store".to_string());
    todo_list.create_todo("Buy food".to_string(), "Buy food from the canteeen".to_string());

    let _ = todo_list.update_todo(1, "Buy drugs".to_string(), "Buy drugs from the pharmacy".to_string());

    let _ = todo_list.get_todo(1);
    let _ = todo_list.get_todo(2);

    todo_list.mark_todo_as_completed(1);
    let _ = todo_list.get_todo(1);

    todo_list.delete_todo(2);
    let _ = todo_list.get_todo(2);    
}
