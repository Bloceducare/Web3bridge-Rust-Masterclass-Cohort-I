use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Category {
    Work,
    Personal,
    Health,
}

#[derive(Debug, Clone)]
enum Status {
    Complete,
    Pending,
    Incomplete,
}

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    task_name: String,
    description: String,
    active: bool,
    category: Category,
    status: Status,
}

impl Category {
    fn from_string(input: &str) -> Result<Category, String> {
        match input.to_lowercase().trim() {
            "work" => Ok(Category::Work),
            "personal" => Ok(Category::Personal),
            "health" => Ok(Category::Health),
            _ => Err("Invalid category. Please enter: work, personal, or health".to_string()),
        }
    }
}

impl Status {
    fn from_string(input: &str) -> Result<Status, String> {
        match input.to_lowercase().trim() {
            "complete" => Ok(Status::Complete),
            "pending" => Ok(Status::Pending),
            "incomplete" => Ok(Status::Incomplete),
            _ => Err("Invalid status. Please enter: complete, pending, or incomplete".to_string()),
        }
    }
}

impl Todo {
    fn new(id: u32, task_name: String, description: String, active: bool, category: Category, status: Status) -> Self {
        Todo {
            id,
            task_name,
            description,
            active,
            category,
            status,
        }
    }

    fn display(&self) {
        println!("\n--- Todo Item ---");
        println!("ID: {}", self.id);
        println!("Task: {}", self.task_name);
        println!("Description: {}", self.description);
        println!("Category: {:?}", self.category);
        println!("Status: {:?}", self.status);
        println!("Active: {}", self.active);
    }

    fn summary(&self) -> String {
        format!("[ID: {}] [{:?}] {} - {:?} ({})", 
                self.id, self.category, self.task_name, self.status, 
                if self.active { "Active" } else { "Inactive" })
    }
}

struct TodoManager {
    todos: HashMap<u32, Todo>,
    next_id: u32,
}

impl TodoManager {
    fn new() -> Self {
        TodoManager {
            todos: HashMap::new(),
            next_id: 1,
        }
    }

    fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

fn create_todo(manager: &mut TodoManager, task_name: String, description: String, 
               active: bool, category: Category, status: Status) -> u32 {
    let id = manager.get_next_id();
    let todo = Todo::new(id, task_name, description, active, category, status);
    
    println!("CREATE TODO");
    println!("Creating new todo with ID: {}", id);
    todo.display();
    
    manager.todos.insert(id, todo);
    println!("Todo created successfully!");
    id
}

fn update_todo(manager: &mut TodoManager, id: u32, task_name: Option<String>, 
               description: Option<String>, active: Option<bool>, 
               category: Option<Category>, status: Option<Status>) -> bool {
    println!("Attempting to update todo with ID: {}", id);
    
    if let Some(todo) = manager.todos.get_mut(&id) {
        if let Some(name) = task_name {
            todo.task_name = name;
        }
        if let Some(desc) = description {
            todo.description = desc;
        }
        if let Some(act) = active {
            todo.active = act;
        }
        if let Some(cat) = category {
            todo.category = cat;
        }
        if let Some(stat) = status {
            todo.status = stat;
        }
        
        println!("Todo updated successfully!");
        todo.display();
        true
    } else {
        println!("Todo with ID {} not found!", id);
        false
    }
}

fn delete_todo(manager: &mut TodoManager, id: u32) -> bool {
    println!("Attempting to delete todo with ID: {}", id);
    
    if let Some(todo) = manager.todos.remove(&id) {
        println!("Deleted todo: {}", todo.summary());
        println!("Todo deleted successfully!");
        true
    } else {
        println!(" Todo with ID {} not found!", id);
        false
    }
}

fn edit_todo(manager: &mut TodoManager, id: u32, new_task_name: String, new_description: String) -> bool {
    println!("EDIT TODO");
    println!("Editing todo with ID: {}", id);
    
    if let Some(todo) = manager.todos.get_mut(&id) {
        println!("Before editing:");
        todo.display();
        
        todo.task_name = new_task_name;
        todo.description = new_description;
        
        println!("After editing:");
        todo.display();
        println!("Todo edited successfully!");
        true
    } else {
        println!("Todo with ID {} not found!", id);
        false
    }
}

fn mark_todo_completed(manager: &mut TodoManager, id: u32) -> bool {
    println!("Marking todo with ID: {} as completed", id);
    
    if let Some(todo) = manager.todos.get_mut(&id) {
        println!("Before marking as complete:");
        println!("Status: {:?}", todo.status);
        
        todo.status = Status::Complete;
        
        println!("After marking as complete:");
        todo.display();
        println!("Todo marked as completed!");
        true
    } else {
        println!(" Todo with ID {} not found!", id);
        false
    }
}

fn display_all_todos(manager: &TodoManager) {
    if manager.todos.is_empty() {
        println!("No todos found.");
        return;
    }
    
    for (_, todo) in &manager.todos {
        println!("{}", todo.summary());
    }
}

fn main() {
    println!("RUST TODO MANAGEMENT SYSTEM");
    
    let mut manager = TodoManager::new();
    
    let id1 = create_todo(&mut manager, 
                         "Complete Rust project".to_string(), 
                         "Build a comprehensive todo application".to_string(), 
                         true, 
                         Category::Work, 
                         Status::Pending);
    
    let id2 = create_todo(&mut manager, 
                         "Go to gym".to_string(), 
                         "30 minutes cardio workout".to_string(), 
                         true, 
                         Category::Health, 
                         Status::Incomplete);
    
    let id3 = create_todo(&mut manager, 
                         "Buy groceries".to_string(), 
                         "Milk, bread, eggs, and vegetables".to_string(), 
                         false, 
                         Category::Personal, 
                         Status::Pending);
    
    display_all_todos(&manager);
    
    update_todo(&mut manager, id2, 
                Some("Go to gym and swimming".to_string()), 
                Some("45 minutes cardio + 30 minutes swimming".to_string()), 
                Some(true), 
                Some(Category::Health), 
                Some(Status::Pending));
    
    edit_todo(&mut manager, id3, 
              "Buy groceries and cook dinner".to_string(), 
              "Buy ingredients and prepare healthy dinner".to_string());
    
    mark_todo_completed(&mut manager, id1);
    
    mark_todo_completed(&mut manager, 999);
    
    println!("\n FINAL STATE OF ALL TODOS:");
    display_all_todos(&manager);
    
    delete_todo(&mut manager, id2);
    
    delete_todo(&mut manager, 999);
    
    println!("\n TODOS AFTER DELETION:");
    display_all_todos(&manager);
    
    println!("\n Program completed successfully!");
}
