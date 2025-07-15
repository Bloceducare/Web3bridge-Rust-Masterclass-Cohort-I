#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    description: Option<String>,
    completed: bool,
}

impl Todo {
    fn create_todo(id: u32, title: String, description: Option<String>) -> Self {
        Todo {
            id,
            title,
            description,
            completed: false,
        }
    }

    fn update_todo(&mut self, title: String, description: Option<String>) {
        self.title = title;
        self.description = description;
    }

    fn edit_todo(&mut self, id: u32, title: String, description: Option<String>) {
        self.id = id;
        self.update_todo(title, description);
        self.completed = false; 
    }

    fn delete_todo(&mut self) {
        self.id = 0;
        self.title.clear();
        self.description = None;
        self.completed = false;
    }
    
    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

fn main() {
    let mut todo = Todo::create_todo(1, String::from("Learn Rust"), Some(String::from("Complete the Rust book")));
    
    println!("Created Todo: {:?}", todo);
    
    todo.update_todo(String::from("Learn Rust Programming"), Some(String::from("Read chapters 4 & 5 only. Don't kill yaself.")));
    println!("Updated Todo: {:?}", todo);
    
    todo.edit_todo(2, String::from("Go to church"), None);
    println!("Edited Todo: {:?}", todo);
    
    todo.mark_completed();
    println!("Marked as completed: {:?}", todo);
    
    todo.delete_todo();
    println!("Deleted Todo: {:?}", todo);
}
