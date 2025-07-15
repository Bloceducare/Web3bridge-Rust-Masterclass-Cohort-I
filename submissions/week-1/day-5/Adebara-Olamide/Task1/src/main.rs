use std::io;
#[derive(Debug)]
struct Item {
    name: String,
    quantity: u32,
    is_done: bool,
}

#[derive(Debug)]
struct Itemlist {
    items: Vec<Item>,
}

impl Itemlist {
    fn add_item(&mut self, name: &str, quantity: u32, is_done: bool) {
        let item = Item {
            name: name.to_string(),
            quantity,
            is_done,
        };
        self.items.push(item);
    }
    fn remove_item(&mut self, name: &str) {
        // Function to remove an item
        self.items.retain(|item| item.name != name);
    }
    fn edit_item(
        &mut self,
        name: &str,
        new_name: Option<&str>,
        new_quantity: Option<u32>,
        new_is_done: Option<bool>
    ) {
        // Function to edit an item
        println!("Item edited!");
        if let Some(item) = self.items.iter_mut().find(|i| i.name == name) {
            if let Some(new_name) = new_name {
                item.name = new_name.to_string();
            }
            if let Some(new_quantity) = new_quantity {
                item.quantity = new_quantity;
            }
            if let Some(new_is_done) = new_is_done {
                item.is_done = new_is_done;
            }
        }
    }
    fn view_items(&self) {
        // Function to view items
        if self.items.is_empty() {
            println!("No item in the list!")
        } else {
            println!("Items in the list:");
            for item in &self.items {
                println!(
                    "Name: {}, Quantity: {}, Done: {}",
                    item.name,
                    item.quantity,
                    item.is_done
                );
            }
        }
    }
}

fn input_field() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the Itemlist App!");
    println!("You can add, remove, edit, and view items in your list.");
    let mut itemlist1 = Itemlist {
        items: Vec::new(),
    };

    loop {
        println!("\nPlease choose an option:");
        println!("1. Add item");
        println!("2. Remove item");
        println!("3. Edit item");
        println!("4. View items");
        println!("5. Exit");

        let choice = input_field();

        match choice.as_str() {
            "1" => {
                println!("Enter the item name:");
                let name = input_field();
                println!("Enter the item quantity:");
                let quantity: u32 = match input_field().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid quantity, setting to 0.");
                        0
                    }
                };
                println!("Is the item done? (true/false):");
                let is_done: bool = input_field().parse().unwrap_or(false);
                itemlist1.add_item(&name, quantity, is_done);
                println!("Item added successfully!");
            }
            "2" => {
                println!("Enter the item name to remove:");
                let name = input_field();
                itemlist1.remove_item(&name);
                println!("Item removed successfully!");
            }
            "3" => {
                println!("Enter the item name to edit:");
                let name = input_field();
                println!("Enter the name of the item (or press Enter to keep current name):");
                let name_input = input_field();
                let new_name = if name_input.is_empty() { None } else { Some(name_input) };
                println!("Enter the new quantity (or press Enter to keep current quantity):");
                let quantity_input = input_field();
                let new_quantity = if quantity_input.is_empty() {
                    None
                } else {
                    match quantity_input.parse::<u32>() {
                        Ok(num) => Some(num),
                        Err(_) => {
                            println!("Invalid quantity, keeping current value.");
                            None
                        }
                    }
                };
                println!("Is the item done? (true/false, or press Enter to keep current value):");
                let is_done_input = input_field();
                let new_is_done = if is_done_input.is_empty() {
                    None
                } else {
                    match is_done_input.parse::<bool>() {
                        Ok(val) => Some(val),
                        Err(_) => {
                            println!("Invalid input, keeping current value.");
                            None
                        }
                    }
                };

                itemlist1.edit_item(&name, new_name.as_deref(), new_quantity, new_is_done);
                println!("Item edited successfully!");
            }
            "4" => {
                println!("Viewing all items:");
                itemlist1.view_items();
            }
            "5" => {
                println!("Exiting the app. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
