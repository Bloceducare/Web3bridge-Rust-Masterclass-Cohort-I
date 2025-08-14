#[derive(Debug, Clone)]
struct TodoItem {
    task: String,
    isCompleted: bool,
    id: i32,
}

enum PURPOSE {
    MarkAsCompleted,
    Update,
    Edit,
    Delete,
}

fn create_todo(mut todo_list: Vec<TodoItem>) -> Vec<TodoItem> {
    let mut input_holder = String::new();

    std::io::stdin()
        .read_line(&mut input_holder)
        .expect("Failed to read line");

    let trimmed_input = input_holder.trim();

    let mut id = 0;

    for (i, _el) in todo_list.iter().enumerate() {
        id = i as i32
    }

    let item = TodoItem {
        task: trimmed_input.to_string(),
        isCompleted: false,
        id: id,
    };
    todo_list.push(item);

    todo_list
}

fn update_todo(mut todo_list: Vec<TodoItem>, todo_id: i32, purpose: PURPOSE) -> Vec<TodoItem> {
    let mut input_holder = String::new();

    std::io::stdin()
        .read_line(&mut input_holder)
        .expect("Failed to read line");

    let trimmed_input = input_holder.trim();

    let mut id = 0;

    let mut new_list = todo_list.clone();
    println!("Updating todo...");

    for (i, el) in todo_list.iter().enumerate() {
        id = i as i32;

        if id == todo_id {
            match purpose {
                PURPOSE::MarkAsCompleted => {
                    new_list[i].isCompleted = true;
                }
                PURPOSE::Edit => new_list[i].task = trimmed_input.to_string(),
                PURPOSE::Update => {
                    new_list[i].task = trimmed_input.to_string();
                }
                PURPOSE::Delete => {
                    new_list.remove(i);
                }
            }
        }
    }

    new_list
}

fn edit_todo(mut todo_list: Vec<TodoItem>, todo_id: i32, purpose: PURPOSE) -> Vec<TodoItem> {
    let mut input_holder = String::new();

    std::io::stdin()
        .read_line(&mut input_holder)
        .expect("Failed to read line");

    let trimmed_input = input_holder.trim();

    let mut id = 0;

    let mut new_list = todo_list.clone();
    println!("editing todo as completed");

    for (i, el) in todo_list.iter().enumerate() {
        id = i as i32;

        if id == todo_id {
            match purpose {
                PURPOSE::MarkAsCompleted => {
                    new_list[i].isCompleted = true;
                }
                PURPOSE::Edit => new_list[i].task = trimmed_input.to_string(),
                PURPOSE::Update => {
                    println!("PURPOSE::Update is same as edit")
                }
                PURPOSE::Delete => {
                    new_list.remove(i);
                }
            }
        }
    }

    new_list
}

fn delete_todo(mut todo_list: Vec<TodoItem>, todo_id: i32, purpose: PURPOSE) -> Vec<TodoItem> {
    let mut input_holder = String::new();

    std::io::stdin()
        .read_line(&mut input_holder)
        .expect("Failed to read line");

    let trimmed_input = input_holder.trim();

    let mut id = 0;

    let mut new_list = todo_list.clone();
    println!("deleting todo..");

    for (i, el) in todo_list.iter().enumerate() {
        id = i as i32;

        if id == todo_id {
            match purpose {
                PURPOSE::MarkAsCompleted => {
                    new_list[i].isCompleted = true;
                }
                PURPOSE::Edit => new_list[i].task = trimmed_input.to_string(),
                PURPOSE::Update => {
                    println!("PURPOSE::Update is same as edit")
                }
                PURPOSE::Delete => {
                    new_list.remove(i);
                }
            }
        }
    }

    new_list
}

fn MarkAsCompleted_todo(
    mut todo_list: Vec<TodoItem>,
    todo_id: i32,
    purpose: PURPOSE,
) -> Vec<TodoItem> {
    let mut input_holder = String::new();

    std::io::stdin()
        .read_line(&mut input_holder)
        .expect("Failed to read line");

    let trimmed_input = input_holder.trim();

    let mut id = 0;

    let mut new_list = todo_list.clone();

    println!("Marking todo as completed...");

    for (i, el) in todo_list.iter().enumerate() {
        id = i as i32;

        if id == todo_id {
            match purpose {
                PURPOSE::MarkAsCompleted => {
                    new_list[i].isCompleted = true;
                }
                PURPOSE::Edit => new_list[i].task = trimmed_input.to_string(),
                PURPOSE::Update => {
                    println!("PURPOSE::Update is same as edit")
                }
                PURPOSE::Delete => {
                    new_list.remove(i);
                }
            }
        }
    }

    new_list
}

fn main() {
    println! {"Creating todo, pls enter todo task.."};
    let mut todo_list: Vec<TodoItem> = Vec::new();

    let mut final_list = create_todo(todo_list);
    println! {"✅ Todo list created"};
    println!("new_todo: {:#?}", &final_list[..]);

    println! {"To edit todo, pls enter new edit.."};
    let edited_todo = edit_todo(final_list.clone(), 0, PURPOSE::Update);
    println! {"✅ Todo list edited"};
    println!("edited_todo Todos: {:#?}", &edited_todo[..]);

    println! {"To update todo, pls enter new update.."};
    let updated_todo = update_todo(final_list.clone(), 0, PURPOSE::Update);
    println! {"✅ Todo list updated"};
    println!("updated_todo Todos: {:#?}", &updated_todo[..]);

    println! {"To mark todo as completed, pls click enter..."};
    let MarkAsCompleted = MarkAsCompleted_todo(final_list.clone(), 0, PURPOSE::MarkAsCompleted);
    println! {"✅ Todo list marked as completed"};
    println!("marked_as_completed {:#?}", &MarkAsCompleted);

    println! {"To delete todo, pls click enter..."};
    let deleted_todo = delete_todo(final_list.clone(), 0, PURPOSE::Delete);
    println! {"✅ Todo list deleted"};
    println!("deleted_todd: {:#?}", &deleted_todo);
}
