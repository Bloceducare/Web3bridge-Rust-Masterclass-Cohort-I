
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Todo {
    id: u32,
    title: String,
    description: String,  
    completed: bool,
}

#[derive(Debug)]
struct TodoList {
    todos: HashMap<u32, Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { todos: HashMap::new() }
    }

    fn add_todo(&mut self, title: String, description: String) {
        let id =  self.todos.len() as u32 + 1;
        self.todos.insert(id, Todo { id, title, description, completed: false });
    }

    fn update_todo(&mut self, id: u32, title: String, description: String) {
        self.todos.insert(id, Todo { id, title, description, completed: false });
    }

    fn delete_todo(&mut self, id: u32) {
        self.todos.remove(&id);
    }

    fn mark_as_completed(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.completed = true;
            true
        } else {
            println!("Todo with id {} not found", id);
            false
        }
    }
}


fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_todo(String::from("Buy groceries"), String::from("Buy groceries"));
    todo_list.add_todo(String::from("Buy food"), String::from("Buy food"));
    todo_list.add_todo(String::from("Buy clothes"), String::from("Buy clothes"));

    println!("Todo List: {:?}", todo_list);

    todo_list.update_todo(1, String::from("Buy food"), String::from("Buy food"));
    println!("Todo List 1 after update: {:?}", todo_list);

    todo_list.delete_todo(2);
    println!("Todo List 2 after delete: {:?}", todo_list);

    todo_list.mark_as_completed(1);
    println!("Todo List 1 after completion: {:?}", todo_list);

    todo_list.mark_as_completed(2);
    println!("Todo List 2 after completion: {:?}", todo_list);


}