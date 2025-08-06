#[warn(dead_code)]
enum TodoLabel {
    Pray,
    Learning,
    WorkOut,
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

    create_todo(&mut todo_list, "Morning Workout".to_string(), "30 minutes cardio and squat training".to_string(), TodoLabel::WorkOut);
    create_todo(&mut todo_list, "Daily Prayer".to_string(), "Morning devotion and gratitude practice".to_string(), TodoLabel::Pray);
    create_todo(&mut todo_list, "Learn Rust Ownership".to_string(), "Study borrowing, references, and lifetimes".to_string(), TodoLabel::Learning);
    create_todo(&mut todo_list, "Project Deadline".to_string(), "Submit my pitch deck by 5PM".to_string(), TodoLabel::Urgent);
    create_todo(&mut todo_list, "Grocery Shopping".to_string(), "Buy ingredients for weekend meal prep".to_string(), TodoLabel::Others);

    show_all_tasks(&todo_list);
    edit_todo(&mut todo_list, 2, "Master Rust Fundamentals".to_string(), "Complete ownership, structs, enums, and error handling".to_string(), TodoLabel::Learning);
    mark_todo_completed(&mut todo_list, 0);
    delete_todo(&mut todo_list, 4);
    show_all_tasks(&todo_list);
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