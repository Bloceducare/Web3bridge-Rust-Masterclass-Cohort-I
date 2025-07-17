
// Todo app
use std::io;
fn main(){
    let mut todo_list = TodoList::new();
    loop {
        println!("Todo List Menu:");
        println!("1. Add Todo Item");
        println!("2. Display All Todo Items");
        println!("3. Mark Item Completed");
        println!("4. Mark Item Incomplete");
        println!("5. Update Item");
        println!("6. Remove Item");
        println!("7. Edit Item");
        println!("8. Exit");

        let choice = TodoList::get_user_input();
        match choice.as_str() {
            "1" => {
                println!("Enter title:");
                let title = TodoList::get_user_input();
                println!("Enter description:");
                let description = TodoList::get_user_input();
                todo_list.add_item(title, description);
            }
            "2" => todo_list.all_todos(),
            "3" => {
                println!("Enter item ID to mark as completed:");
                let id: u32 = TodoList::get_user_input().parse().unwrap_or(0);
                todo_list.mark_item_completed(id);
            }
            "4" => {
                println!("Enter item ID to mark as incomplete:");
                let id: u32 = TodoList::get_user_input().parse().unwrap_or(0);
                todo_list.mark_item_incomplete(id);
            }
            "5" => {
                println!("Enter item ID to update:");
                let id: u32 = TodoList::get_user_input().parse().unwrap_or(0);
                todo_list.update_item(id, String::new(), String::new());
            }
            "6" => {
                println!("Enter item ID to remove:");
                let id: u32 = TodoList::get_user_input().parse().unwrap_or(0);
                todo_list.remove_item(id);
            }
            "7" => {
                println!("Enter item ID to edit:");
                let id: u32 = TodoList::get_user_input().parse().unwrap_or(0);
                todo_list.edit_item(id);
            }
            "8" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}
#[derive(Debug, Clone)]
struct TodoItem {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl TodoItem {
    fn new(id: u32, title: String, description: String) -> Self {
        TodoItem {
            id,
            title,
            description,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
    fn mark_incomplete(&mut self) {
        self.completed = false;
    }
    fn update_title(&mut self, new_title: String, new_description: String) {
        self.title = new_title;
        self.description = new_description;
    }

    fn display_summary(&self) {
        let status = if self.completed { "Completed" } else { "Not Completed" };
        println!("ID: {}, Title: {}, Status: {}", self.id, self.title, status);
    }
}

#[derive(Debug, Clone)]
    struct TodoList {
        items: Vec<TodoItem>,
        id_counter: u32,
    }

    impl TodoList {
        fn new() -> Self {
            TodoList {
                items: Vec::new(),
                id_counter: 1,
            }
        }

        fn add_item(&mut self, title: String, description: String) {
            let new_item = TodoItem::new(self.id_counter, title, description);
            self.items.push(new_item);
            self.id_counter += 1;
        }

        fn display_items(&self) {
            for item in &self.items {
                item.display_summary();
            }
        }
        fn mark_item_completed(&mut self, id: u32) {
            if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                item.mark_completed();
            } else {
                println!("Item with ID {} not found.", id);
            }
        }
        fn mark_item_incomplete(&mut self, id: u32) {
            if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                item.mark_incomplete();
            } else {
                println!("Item with ID {} not found.", id);
            }
        }
        fn update_item(&mut self, id: u32, new_title: String, new_description: String) {
            if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                item.update_title(new_title, new_description);
            } else {
                println!("Item with ID {} not found.", id);
            }
        }
        fn remove_item(&mut self, id: u32) {
            if let Some(pos) = self.items.iter().position(|i| i.id == id) {
                self.items.remove(pos);
            } else {
                println!("Item with ID {} not found.", id);
            }
        }

        fn edit_item(&mut self, id: u32) {
            if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
                println!("Editing item: {}", item.title);
                println!("Enter new title:");
                let mut new_title = String::new();
                io::stdin().read_line(&mut new_title).unwrap();
                println!("Enter new description:");
                let mut new_description = String::new();
                io::stdin().read_line(&mut new_description).unwrap();
                item.update_title(new_title.trim().to_string(), new_description.trim().to_string());
            } else {
                println!("Item with ID {} not found.", id);
            }
        }

        fn all_todos(&self) {
            if self.items.is_empty() {
                println!("No todo items found.");
            } else {
                println!("All Todo Items:");
                self.display_items();
            }
        }

        fn get_user_input() -> String {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().to_string()
        }
    }
