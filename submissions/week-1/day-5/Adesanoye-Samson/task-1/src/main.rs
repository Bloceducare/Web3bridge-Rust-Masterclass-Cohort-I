use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Todo {
    id: u32,
    title: String,
    is_completed: bool, 
}

#[derive(Debug)]
struct TodoList {
    todos: HashMap<u32, Todo>
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { todos: HashMap::new() }
    }

    fn add_todo(&mut self, title: String) {
        let id = self.todos.len() as u32 +1;
        self.todos.insert(id, Todo {id, title, is_completed: false });
    }

    fn update_todo(&mut self, id: u32, title: String) {
        self.todos.insert(id, Todo { id, title, is_completed: false });
    }

    fn delete_todo(&mut self, id: u32) {
        self.todos.remove(&id);
    }

    fn mark_as_completed(&mut self, id: u32) -> bool {
        if let Some(todo) = self.todos.get_mut(&id) {
            todo.is_completed = true;
            true
        } else {
            println!("Todo with id {} not found", id);
            false
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_todo(String::from("Buy clothes"));

    println!("Todo List: {:?}", todo_list);

    todo_list.update_todo(1, String::from("Buy food"));
    println!("Todo List 1 after update: {:?}", todo_list);

    todo_list.mark_as_completed(1);
    println!("Todo List 1 after completion: {:?}", todo_list);

    todo_list.delete_todo(1);
    println!("Todo List 1 after delete: {:?}", todo_list);
}
