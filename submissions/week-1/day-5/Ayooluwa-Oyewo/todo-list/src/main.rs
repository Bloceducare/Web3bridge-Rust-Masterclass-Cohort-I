/**
Task 1: Todo list in Rust
Objective
Write a Rust program with separate functions to:

Create Todo
Update Todo
Delete Todo
Edit Todo
Mark todo as completed
Then you should attach a screenshot of the printed version, just one image containing the result of the logs of that function.
Attach the screenshot to the code and Push your code to github also, don't submit screenshot alone.
Then call each of them in main() with values and run the program using cargo run.
 */

#[allow(dead_code)]
#[derive(Debug)]
struct Todo {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}
struct InputData {
    id: i32,
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

fn create_todo(data: InputData) -> Todo {
    Todo {
        id: data.id,
        title: data.title.unwrap_or_default(),
        description: data.description.unwrap_or_default(),
        completed: data.completed.unwrap_or(false),
    }
}

fn update_todo(todos: &mut Vec<Todo>, data: InputData) {
    let todo = todos.iter_mut().find(|t| t.id == data.id);
    if let Some(todo) = todo {
        todo.title = data.title.unwrap_or_default();
        todo.description = data.description.unwrap_or_default();
        todo.completed = data.completed.unwrap_or(false);
    } else {
        println!("Todo not found");
    }
}

fn mark_todo_as_completed(todos: &mut Vec<Todo>, id: i32) {
    let todo = todos.iter_mut().find(|t| t.id == id);
    if let Some(todo) = todo {
        todo.completed = true;
    } else {
        println!("Todo not found");
    }
}

fn edit_todo(todos: &mut Vec<Todo>, data: InputData) {
    let todo = todos.iter_mut().find(|t| t.id == data.id);
    if let Some(todo) = todo {
        todo.title = data.title.unwrap_or_default();
        todo.description = data.description.unwrap_or_default();
        todo.completed = data.completed.unwrap_or(false);
    } else {
        println!("Todo not found");
    }
}

fn delete_todo(todos: &mut Vec<Todo>, id: i32) {
    if let Some(index) = todos.iter().position(|t| t.id == id) {
        todos.remove(index);
    } else {
        println!("Todo not found");
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    // Create todo
    let todo = create_todo(InputData {
        id: 1,
        title: Some("Buy groceries".to_string()),
        description: Some("Buy groceries from the store".to_string()),
        completed: None,
    });
    todos.push(todo);
    println!("created todo: {:?}", todos);

    // Update todo
    update_todo(
        &mut todos,
        InputData {
            id: 1,
            title: Some("Buy groceries updated".to_string()),
            description: Some("Buy groceries from the store updated".to_string()),
            completed: Some(true),
        },
    );
    println!("updated todo: {:?}", todos);

    // Mark todo as completed
    mark_todo_as_completed(&mut todos, 1);
    println!("marked todo as completed: {:?}", todos);

    // Edit todo
    edit_todo(
        &mut todos,
        InputData {
            id: 1,
            title: Some("Buy groceries edited".to_string()),
            description: Some("Buy groceries from the store edited".to_string()),
            completed: None,
        },
    );
    println!("edited todo: {:?}", todos);

    // Delete todo
    delete_todo(&mut todos, 1);
    println!("deleted todo: {:?}", todos);
}
