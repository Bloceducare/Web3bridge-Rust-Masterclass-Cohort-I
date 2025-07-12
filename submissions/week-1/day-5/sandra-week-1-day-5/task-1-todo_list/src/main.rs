
#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

fn create_todo(todos: &mut Vec<Todo>, title: String, id: u32) {
    let todo = Todo {
        id,
        title,
        completed: false,
    };
    todos.push(todo);
    println!("Created Todo");
    print_todos(todos);
}

fn update_todo(todos: &mut Vec<Todo>, id: u32, new_title: String) {
    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.title = new_title;
            println!("Updated Todo with ID {id}");
            break;
        }
    }
    print_todos(todos);
}

fn edit_todo(todos: &mut Vec<Todo>, id: u32, new_title: String) {
    update_todo(todos, id, new_title);
}

fn delete_todo(todos: &mut Vec<Todo>, id: u32) {
    todos.retain(|todo| todo.id != id);
    println!("Deleted Todo with ID {id}");
    print_todos(todos);
}

fn mark_completed(todos: &mut Vec<Todo>, id: u32) {
    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.completed = true;
            println!(" Marked Todo with ID {id} as completed");
            break;
        }
    }
    print_todos(todos);
}

fn print_todos(todos: &Vec<Todo>) {
    println!("\n📋 Current Todo List:");
    for todo in todos {
        println!(
            "ID: {}, Title: '{}', Completed: {}",
            todo.id,
            todo.title,
            todo.completed
        );
    }
    println!("----------------------------\n");
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    create_todo(&mut todos, "Learn Rust".to_string(), 1);
    create_todo(&mut todos, "Build CLI app".to_string(), 2);

    update_todo(&mut todos, 1, "Learn Rust - Ownership".to_string());

    mark_completed(&mut todos, 1);

    delete_todo(&mut todos, 2);
    println!("todos: {:?}", todos);
}
