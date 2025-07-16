use colored::Colorize;
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
enum TodoTags {
    Personal,
    Work,
    School,
    Shopping,
    Entertainment,
    Spiritual,
}

#[derive(Debug)]
struct Todo {
    id: Uuid,
    title: String,
    tags: TodoTags,
    is_completed: bool,
}

// AI suggested this to me when I asked for suggested improvements. Every other part of the code is unique
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Todo {{ id: {}, title: '{}', tags: {:?}, completed: {} }}",
            self.id, self.title, self.tags, self.is_completed
        )
    }
}

impl Todo {
    fn create(title: String, tags: TodoTags) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            title,
            tags,
            is_completed: false,
        }
    }

    fn update(&mut self, title: String, tags: TodoTags) {
        self.title = title;
        self.tags = tags;
        self.is_completed = false;
    }

    fn completed(&mut self, is_completed: bool) {
        self.is_completed = is_completed;
    }
}

fn create_new_todo(todo_map: &mut HashMap<Uuid, Todo>, title: &str, tags: TodoTags) -> Uuid {
    let new_todo = Todo::create(title.to_string(), tags);
    let id = new_todo.id;
    todo_map.insert(new_todo.id, new_todo);
    id
}

fn update_todo(
    todo_map: &mut HashMap<Uuid, Todo>,
    todo_id: Uuid,
    todo_title: &str,
    todo_tags: TodoTags,
) {
    todo_map
        .entry(todo_id)
        .and_modify(|entry| entry.update(todo_title.to_string(), todo_tags));
}

fn mark_todo_as_completed(todo_map: &mut HashMap<Uuid, Todo>, todo_id: Uuid) {
    todo_map
        .entry(todo_id)
        .and_modify(|entry| entry.completed(true));
}

fn delete_todo(todo_map: &mut HashMap<Uuid, Todo>, todo_id: &Uuid) {
    todo_map.remove(todo_id);
}

fn print_todos(todo_map: &HashMap<Uuid, Todo>) {
    println!("{}", "Todo List: ".blue());
    for todo in todo_map.values() {
        println!("{}", todo);
    }
}

fn main() {
    let mut todo_list: HashMap<Uuid, Todo> = HashMap::new();

    println!("{}", "\n==== Creating Todo One and Two ====".blue());
    let todo_one = create_new_todo(&mut todo_list, "Take the trash out", TodoTags::Personal);
    let todo_two = create_new_todo(&mut todo_list, "Pray", TodoTags::Spiritual);

    print_todos(&todo_list);

    println!("{}", "\n==== Updating Todo One ====".yellow());
    update_todo(
        &mut todo_list,
        todo_one,
        "Sweep the House",
        TodoTags::Personal,
    );
    print_todos(&todo_list);

    println!("{}", "\n==== Marking Todo Two as Completed ====".green());
    mark_todo_as_completed(&mut todo_list, todo_two);
    print_todos(&todo_list);

    println!("{}", "\n==== Deleting Todo One ====".red());
    delete_todo(&mut todo_list, &todo_one);
    print_todos(&todo_list);
}
