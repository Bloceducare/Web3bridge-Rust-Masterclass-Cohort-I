#[derive(Clone, Debug)]
struct Todo {
    task: String,
    status: bool,
}

impl Todo {
    fn new(task: String) -> Self {
        Self {
            task,
            status: false,
        }
    }

    fn edit_task(&mut self, task: String) {
        self.task = task;
    }

    fn toggle_status(&mut self) {
        self.status = !self.status;
    }
}

fn delete_todo(todos: &mut Vec<&Todo>, index: usize) {
    if index < todos.len() {
        todos.remove(index);
    }
}

fn main() {
    let mut todos: Vec<&Todo> = Vec::new();

    let mut todo1 = Todo::new(String::from("Do laundry"));
    todo1.edit_task(String::from("Do laundry and iron"));
    todo1.toggle_status();
    todo1.toggle_status();
    todo1.toggle_status();

    let mut todo2 = Todo::new(String::from("Buy groceries"));
    todo2.edit_task(String::from("Buy ggs and iron"));
    todo2.toggle_status();
    todo2.toggle_status();

    todos.push(&todo1);
    todos.push(&todo2);
    todos.push(&todo1);
    todos.push(&todo2);

    delete_todo(&mut todos, 3);

    println!("First todo: {todo1:#?}");
    println!("\n");
    println!("Second todo: {todo2:#?}");

    println!("=============================================");
    println!("\n");

    println!("All todos: ");
    let mut i = 1;
    for todo in todos {
        println!("{}: {todo:#?}", i);
        println!("\n");
        i = i + 1;
    }
}
