#[derive(Debug)]
struct Todo {
    id: u8,
    name: String,
    description: String,
    completed: bool,
}

let mut todos: Vec<Todo> = Vec::new();

fn create_todo(name: String, description: String) -> Todo {
    Todo {
        id: todos.len() as u8 + 1,
        name,
        description,
        completed: false,
    }
}

fn update_todo(
    id: u8,
    todo: &mut Todo,
    name: String,
    description: String,
    completed: Option<bool>,
) -> &mut Todo {

    todos.iter_mut().find(|t| t.id==id).map_or_else(||{
        panic!("Todo with id {} not found", id)
    }, |todo| {
        todo.name = name;
        todo.description = description;
        if let Some(completed_status) = completed {
            todo.completed = completed_status;
        }
        todo
    })
}

fn delete_todo(id: u8) {
    if let Some(pos) = todos.iter().position(|t| t.id == id) {
        todos.remove(pos);
    } else {
        panic!("Todo with id {} not found", id);
    }
}

fn edit_todo(id: u8, name: String, description: String, completed: Option<bool>) -> &mut Todo {
    update_todo(id, &mut todos[id as usize - 1], name, description, completed)
}



fn main(){
    let todo1 = create_todo("Rusts learning".into(),"Learn Rust programming language".into());
    let todo2 = create_todo("Web3 learning".into(),"Learn Web3 technologies".into());
    todos.push(todo1);
    todos.push(todo2);

    println!("Todos after creation: {:?}", todos);  
    let updated_todo = edit_todo(1, "Advanced".into(), "concepts in Rust".into(), Some(true));
    println!("Todos after update: {:?}", todos);
    delete_todo(2);
    println!("Todos after deletion: {:?}", todos);  
    println!("Updated Todo: {:?}", updated_todo);
    println!("Final Todos: {:?}", todos);
    println!("All operations completed successfully.");


}
