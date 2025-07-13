#[derive(Debug)]
struct MyTodo {
    title: String,
    completed: bool,
}


fn create_todo(title: &str) -> MyTodo {
    MyTodo {
        title: title.to_string(),
        completed: false,
    }
}

fn edit_todo(todo: &mut MyTodo, new_title: &str) {
    todo.title = new_title.to_string();
    println!("Edited Todo ==> {:?}", todo);
}


fn completed(todo: &mut MyTodo) {
    todo.completed = true;
    println!("Todo marked as completed ==> {:?}", todo);
}


fn delete_todo(todo: &mut MyTodo) {
    todo.title = String::from("[Deleted]");
    println!("Deleted Todo ==> {:?}", todo);
}

fn main() {
    
    let mut my_todo = create_todo("Currently learning Rust");
    println!("Created Todo ==> {:?}", my_todo);

    
    edit_todo(&mut my_todo, "Rust todo");

    
    completed(&mut my_todo);

   
    delete_todo(&mut my_todo);
}
