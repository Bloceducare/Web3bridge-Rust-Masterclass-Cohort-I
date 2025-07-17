#[warn(dead_code)]
enum TodoLabel{
    Personal,
    Work,
    Emergency,
    Shopping,
    Urgent,
    Others
}

struct Todo {
    title: String,
    description: String,
    completed: bool,
    label: TodoLabel,
}

fn main() {
    let mut todo_list: Vec<Todo> = Vec::new();

    create_todo(&mut todo_list, "Buy groceries".to_string(), "Buy milk, eggs, and bread".to_string(), TodoLabel::Shopping);
    create_todo(&mut todo_list, "Clean the room".to_string(), "Organize and vacuum the living room".to_string(), TodoLabel::Personal);
    create_todo(&mut todo_list, "Learn Rust".to_string(), "Complete the Rust tutorial".to_string(), TodoLabel::Work);
    create_todo(&mut todo_list, "Prepare for meeting".to_string(), "Gather documents and notes".to_string(), TodoLabel::Work);

    println!("===========================================================");
    show_all_tasks(&todo_list);
    println!("===========================================================");
    edit_todo(&mut todo_list, 2, "Clean and organize the room".to_string(), "Organize and vacuum the living room thoroughly".to_string(), TodoLabel::Personal);
    println!("===========================================================");
    mark_todo_completed(&mut todo_list, 3);
    println!("===========================================================");
    delete_todo(&mut todo_list, 1);
    println!("===========================================================");
    show_all_tasks(&todo_list);
    println!("===========================================================");
}


fn create_todo(todo_list: &mut Vec<Todo>, title: String, description: String, label: TodoLabel) {
    let todo = Todo {
        title,
        description,
        completed: false,
        label,
    };
    println!("Added: {}", todo.title);
    todo_list.push(todo);
}

fn edit_todo(todo_list: &mut Vec<Todo>, index: usize, new_title: String, new_description: String, new_label: TodoLabel) {
    if let Some(todo) = todo_list.get_mut(index) {
        todo.title = new_title;
        todo.description = new_description;
        todo.label = new_label;
        println!("Todo at index {} updated to '{}'", index, todo.title);
    } else {
        println!("Todo not found at index {}", index);
    }
}

fn mark_todo_completed(todo_list: &mut Vec<Todo>, index: usize) {
    if let Some(todo) = todo_list.get_mut(index) {
        todo.completed = true;
        println!("Todo at index {} marked as completed: {}", index, todo.title);
    } else {
        println!("Todo not found at index {}", index);
    }
}

fn delete_todo(todo_list: &mut Vec<Todo>, index: usize) {
    if index < todo_list.len() {
        todo_list.remove(index);
        println!("Todo at index {} deleted.", index);
    } else {
        println!("Todo not found at index {}", index);
    }
}

fn show_all_tasks(todo_list: &Vec<Todo>) {
    if todo_list.is_empty() {
        println!("No tasks available.");
    } else {
        for (index, todo) in todo_list.iter().enumerate() {
            let status = if todo.completed { "Done" } else { "Pending" };
            println!("{}: {} - {} [{}]", index, todo.title, todo.description, status);
        }
    }
}

