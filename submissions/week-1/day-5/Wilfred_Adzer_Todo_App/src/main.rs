#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    name: String,
    desc: String,
    is_completed: bool,
}

fn create_todo(id: u32, name: &str, desc: &str) -> Todo {
    let todo = Todo {
        id,
        name: name.to_string(),
        desc: desc.to_string(),
        is_completed: false,
    };
    println!("[CREATE] Todo created: {:?}", todo);
    todo
}

fn update_todo(todo: &mut Todo, new_name: Option<&str>, new_desc: Option<&str>) {
    if let Some(name) = new_name {
        todo.name = name.to_string();
    }
    if let Some(desc) = new_desc {
        todo.desc = desc.to_string();
    }
    println!("[UPDATE] Todo updated: {:?}", todo);
}

fn delete_todo(todos: &mut Vec<Todo>, id: u32) {
    todos.retain(|todo| todo.id != id);
    println!("[DELETE] Todo with id {} deleted", id);
}

fn edit_todo(todo: &mut Todo, name: &str, desc: &str) {
    todo.name = name.to_string();
    todo.desc = desc.to_string();
    println!("[EDIT] Todo edited: {:?}", todo);
}

fn mark_as_completed(todo: &mut Todo) {
    todo.is_completed = true;
    println!("[COMPLETE] Todo marked as completed: {:?}", todo);
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    // Create
    let mut todo1 = create_todo(1, "Buy groceries", "Milk, eggs, and bread");
    todos.push(todo1.clone());

    // Update
    update_todo(&mut todo1, Some("Buy groceries and fruits"), None);
    todos[0] = todo1.clone();

    // Edit
    edit_todo(&mut todo1, "Go shopping", "Clothes and shoes");
    todos[0] = todo1.clone();

    // Mark completed
    mark_as_completed(&mut todo1);
    todos[0] = todo1.clone();

    // Delete
    delete_todo(&mut todos, 1);

    println!("\nFinal Todos List: {:?}", todos);
}
