#[derive(Debug, Clone)]

struct Todo {
    task: String,
    id: u32,
    completed: bool,
}

 fn create_todo(id: u32, task: String) -> Todo  {
    let todo = Todo {
        id,
        task,
        completed: false,
    };
    println!("\nCreated Todo:\n{:?}", todo);

    todo
}

fn update_todo(todo: &mut Todo, new_task: String) {
    todo.task = new_task;
    println!("\nUpdated todo: \n{:?}", todo);
}

fn delete_todo(todos: &mut Vec<Todo>, id: u32) {
    for (index, todo) in todos.iter().enumerate() {
        if todo.id == id {
            todos.remove(index);
            println!("\n Deleted todo with id: {}", id);
            return;
        }
    }

    println!("\nTodo with id {} not found!", id);
}

fn mark_completed(todo: &mut Todo) {
    todo.completed = true;
    println!("\nCompleted todo: \n{:?}", todo);
}
 

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    let mut id_counter = 1;

    let mut todo1 = create_todo(id_counter, String::from("Wash plates"));
    todos.push(todo1.clone());
    id_counter += 1;

    let todo2 = create_todo(id_counter, String::from("Iron clothes"));
    todos.push(todo2.clone());
    id_counter += 1;

    let todo3 = create_todo(id_counter, String::from("Iron clothes"));
    todos.push(todo3.clone());

    update_todo(&mut todo1, String::from("Wash dishes properly"));
    // Add updated todo back to list
    todos[0] = todo1.clone();

    mark_completed(&mut todo1);

    delete_todo(&mut todos, 1);

    println!("\n📋 Final Todo List:");
    if todos.is_empty() {
        println!("\n (No tasks found)");
    } else {
        for todo in &todos {
            println!("\n {:?}", todo);
        }
    }
}
