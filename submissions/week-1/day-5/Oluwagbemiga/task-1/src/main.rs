#[derive(Debug)]
struct Todo {
    title: String,
    date: String,
    completed: bool,
}

impl Todo {
    fn new(title: String, date: String) -> Self {
        Todo {
            title,
            date,
            completed: false,
        }
    }
    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn update(&mut self, new_title: String, new_date: String) {
        self.title = new_title;
        self.date = new_date;
    }
}

#[derive(Debug)]
struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    fn add(&mut self, title: String, date: String) {
        self.todos.push(Todo::new(title, date));
    }
    fn remove(&mut self, index: usize) {
        if index < self.todos.len() {
            self.todos.remove(index);
        } else {
            println!("No todo found at index {}", index);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    todo_list.add("Buy Food".to_string(), "2023-10-01".to_string());
    todo_list.add("Code".to_string(), "2023-10-02".to_string());

    println!("Initial Todos: {:#?}", todo_list.todos);

    match todo_list.todos.get_mut(0) {
        Some(todo) => {
            todo.mark_completed();
            todo.update("Bought drinks".to_string(), "2023-10-03".to_string())
        }
        None => println!("No todo found to update"),
    }
    println!("Todos after marking completed: {:#?}", todo_list.todos);

    match todo_list.todos.get_mut(1) {
        Some(todo) => todo_list.remove(1),
        None => println!("No todo found to delete"),
    }

    println!("Todos after Deleting: {:#?}", todo_list.todos);
}
