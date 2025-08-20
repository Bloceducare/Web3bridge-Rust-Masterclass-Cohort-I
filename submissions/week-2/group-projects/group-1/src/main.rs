use std::io::{self, Write};

mod lib;
use lib::{InventoryManager, InventoryError};

struct InputHandler;

impl InputHandler {
    fn get_input(prompt: &str) -> Result<String, io::Error> {
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn get_number_input(prompt: &str) -> Result<u32, Box<dyn std::error::Error>> {
        loop {
            let input = Self::get_input(prompt)?;
            match input.parse::<u32>() {
                Ok(num) => return Ok(num),
                Err(_) => println!("Invalid number. Please enter a positive whole number."),
            }
        }
    }

    fn get_price_input(prompt: &str) -> Result<f64, Box<dyn std::error::Error>> {
        loop {
            let input = Self::get_input(prompt)?;
            match input.parse::<f64>() {
                Ok(price) if price >= 0.0 => return Ok(price),
                Ok(_) => println!("Price cannot be negative."),
                Err(_) => println!("Invalid price. Please enter a valid decimal number."),
            }
        }
    }

    fn get_optional_input(field_name: &str, current_value: &str) -> Result<String, io::Error> {
        let input = Self::get_input(&format!("{} (current: {}): ", field_name, current_value))?;
        if input.is_empty() {
            Ok(current_value.to_string())
        } else {
            Ok(input)
        }
    }

    fn get_optional_number_input(field_name: &str, current_value: u32) -> Result<u32, Box<dyn std::error::Error>> {
        let input = Self::get_input(&format!("{} (current: {}): ", field_name, current_value))?;
        if input.is_empty() {
            Ok(current_value)
        } else {
            match input.parse::<u32>() {
                Ok(num) => Ok(num),
                Err(_) => {
                    println!("Invalid number, keeping current value.");
                    Ok(current_value)
                }
            }
        }
    }

    fn get_optional_price_input(field_name: &str, current_value: f64) -> Result<f64, Box<dyn std::error::Error>> {
        let input = Self::get_input(&format!("{} (current: ${:.2}): ", field_name, current_value))?;
        if input.is_empty() {
            Ok(current_value)
        } else {
            match input.parse::<f64>() {
                Ok(price) if price >= 0.0 => Ok(price),
                Ok(_) => {
                    println!("Invalid price (cannot be negative), keeping current value.");
                    Ok(current_value)
                }
                Err(_) => {
                    println!("Invalid price format, keeping current value.");
                    Ok(current_value)
                }
            }
        }
    }

    fn get_confirmation(prompt: &str) -> Result<bool, io::Error> {
        let input = Self::get_input(prompt)?;
        Ok(input.to_lowercase().starts_with('y'))
    }
}

struct UI;

impl UI {
    fn display_menu() {
        // println!("\n" + &"=".repeat(50));
        println!("       INVENTORY STOCK MANAGER");
        println!("{}", "=".repeat(50));
        println!("1. Add Item       (Stage 1)");
        println!("2. View Inventory (Stage 1)");
        println!("3. Remove Item    (Stage 2)");
        println!("4. Edit Item      (Stage 3)");
        println!("5. Exit");
        println!("{}", "=".repeat(50));
    }

    fn display_items(manager: &InventoryManager) {
        println!("\n--- Current Inventory ---");
        
        if manager.is_empty() {
            println!("No items in inventory.");
            return;
        }

        println!("Total items: {}", manager.item_count());
        println!("{:-<60}", "");
        
        let items = manager.get_all_items();
        for (index, item) in items.iter().enumerate() {
            println!("{}. {}", index + 1, item.display());
        }
        
        println!("{:-<60}", "");
        println!("Total inventory value: ${:.2}", manager.total_inventory_value());
    }

    fn display_error(error: &dyn std::error::Error) {
        println!("Error: {}", error);
    }

    fn display_success(message: &str) {
        println!("✓ {}", message);
    }
}

struct App {
    manager: InventoryManager,
}

impl App {
    fn new() -> Self {
        App {
            manager: InventoryManager::new(),
        }
    }

    fn add_item(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n--- Add New Item ---");
        
        let name = InputHandler::get_input("Enter item name: ")?;
        let quantity = InputHandler::get_number_input("Enter quantity: ")?;
        let price = InputHandler::get_price_input("Enter price: $")?;

        match self.manager.add_item(name, quantity, price) {
            Ok(()) => UI::display_success("Item added successfully!"),
            Err(e) => UI::display_error(&e),
        }
        
        Ok(())
    }

    fn view_items(&self) {
        UI::display_items(&self.manager);
    }

    fn remove_item(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n--- Remove Item ---");
        
        if self.manager.is_empty() {
            UI::display_error(&InventoryError::EmptyInventory);
            return Ok(());
        }

        self.view_items();
        let name = InputHandler::get_input("\nEnter the name of the item to remove: ")?;

        match self.manager.find_item(&name) {
            Some(item) => {
                println!("Found item: {}", item.display());
                if InputHandler::get_confirmation("Are you sure you want to remove this item? (y/n): ")? {
                    match self.manager.remove_item(&name) {
                        Ok(_) => UI::display_success("Item removed successfully!"),
                        Err(e) => UI::display_error(&e),
                    }
                } else {
                    println!("Remove operation cancelled.");
                }
            }
            None => UI::display_error(&InventoryError::ItemNotFound),
        }
        
        Ok(())
    }

    fn edit_item(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n--- Edit Item ---");
        
        if self.manager.is_empty() {
            UI::display_error(&InventoryError::EmptyInventory);
            return Ok(());
        }

        self.view_items();
        let original_name = InputHandler::get_input("\nEnter the name of the item to edit: ")?;

        let original_item = match self.manager.find_item(&original_name) {
            Some(item) => item.clone(),
            None => {
                UI::display_error(&InventoryError::ItemNotFound);
                return Ok(());
            }
        };

        println!("Current item details: {}", original_item.display());
        println!("\nEnter new values (press Enter to keep current value):");

        let new_name = InputHandler::get_optional_input("New name", &original_item.name)?;
        let new_quantity = InputHandler::get_optional_number_input("New quantity", original_item.quantity)?;
        let new_price = InputHandler::get_optional_price_input("New price", original_item.price)?;

        println!("\nOriginal: {}", original_item.display());
        println!("Updated:  Name: {}, Quantity: {}, Price: ${:.2}", new_name, new_quantity, new_price);
        
        if InputHandler::get_confirmation("\nSave changes? (y/n): ")? {
            match self.manager.edit_item(&original_name, new_name, new_quantity, new_price) {
                Ok(_) => UI::display_success("Item updated successfully!"),
                Err(e) => UI::display_error(&e),
            }
        } else {
            println!("Edit cancelled. No changes made.");
        }
        
        Ok(())
    }

    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Welcome to the Inventory Stock Manager!");
        
        loop {
            UI::display_menu();
            
            let choice = InputHandler::get_input("Select an option (1-5): ")?;
            
            match choice.as_str() {
                "1" => self.add_item()?,
                "2" => self.view_items(),
                "3" => self.remove_item()?,
                "4" => self.edit_item()?,
                "5" => {
                    println!("Thank you for using Inventory Stock Manager!");
                    break;
                }
                _ => println!("Invalid option. Please select 1-5."),
            }
            
            println!("\nPress Enter to continue...");
            let _ = InputHandler::get_input("");
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.run()
}
