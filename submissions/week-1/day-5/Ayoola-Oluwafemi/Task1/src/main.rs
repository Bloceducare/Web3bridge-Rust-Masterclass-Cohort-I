use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

struct TodoList {
    next_id: u32,
    items: HashMap<u32, Todo>,
}

impl TodoList {
    /// Start with an empty list.
    fn new() -> Self {
        Self {
            next_id: 1,
            items: HashMap::new(),
        }
    }

    /// Create a new to‑do. Returns its ID.
    fn create_todo(&mut self, title: &str) -> u32 {
        let todo = Todo {
            id: self.next_id,
            title: title.to_string(),
            completed: false,
        };
        self.items.insert(self.next_id, todo);
        self.next_id += 1;
        self.next_id - 1
    }

    /// Update *everything* on a to‑do (both title and completed flag).
    fn update_todo(&mut self, id: u32, new_title: &str, completed: bool) -> bool {
        if let Some(todo) = self.items.get_mut(&id) {
            todo.title = new_title.to_string();
            todo.completed = completed;
            true
        } else {
            false
        }
    }

    /// Edit **only the title** of a to‑do.
    fn edit_todo(&mut self, id: u32, new_title: &str) -> bool {
        if let Some(todo) = self.items.get_mut(&id) {
            todo.title = new_title.to_string();
            true
        } else {
            false
        }
    }

    /// Mark a to‑do as completed.
    fn mark_completed(&mut self, id: u32) -> bool {
        if let Some(todo) = self.items.get_mut(&id) {
            todo.completed = true;
            true
        } else {
            false
        }
    }

    /// Delete a to‑do from the list.
    fn delete_todo(&mut self, id: u32) -> bool {
        self.items.remove(&id).is_some()
    }

    /// Helper to print the whole list.
    fn print_all(&self) {
        println!("Current list:");
        for todo in self.items.values() {
            println!("{todo:?}");
        }
        println!("---------------------------");
    }
}

fn main() {
    let mut list = TodoList::new();

    // 1️⃣  Create Todo
    let id = list.create_todo("Finish Rust practice");
    println!("Created todo with id {id}");
    list.print_all();

    // 2️⃣  Update Todo (change title + mark completed in one go)
    list.update_todo(id, "Finish Rust practice ASAP", true);
    println!("Updated todo {id}");
    list.print_all();

    // 3️⃣  Edit Todo (change only the title)
    list.edit_todo(id, "Finish Rust practice by tonight");
    println!("Edited todo {id}");
    list.print_all();

    // 4️⃣  Mark Todo as completed (again, just to demo the separate call)
    list.mark_completed(id);
    println!("Marked todo {id} as completed");
    list.print_all();

    // 5️⃣  Delete Todo
    list.delete_todo(id);
    println!("Deleted todo {id}");
    list.print_all();
}
