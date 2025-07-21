pub mod functions;
pub mod inventory;
pub mod test;

use crate::inventory::*;
use std::io::{self, Write};

fn main() {
    let mut inventory = Inventory::new();

    println!("Group24 Inventory Reorder System!");
    println!("========GROUP24===========");

    loop {
        display_menu();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let choice = input.trim();

        match choice {
            "1" => create_order_cli(&mut inventory),
            "2" => view_order_cli(&inventory),
            "3" => view_all_orders_cli(&inventory),
            "4" => remove_fulfilled_orders_cli(&mut inventory),
            "5" => edit_order_cli(&mut inventory),
            "6" => {
                println!("Thank you for using Inventory Reorder System!");
                break;
            }
            _ => println!("Invalid option! Please choose 1-6."),
        }

        println!(); // space
    }
}

fn display_menu() {
    println!("choose an option:");
    println!("1. Create");
    println!("2. View Order by ID");
    println!("3. View All Orders");
    println!("4. Remove Fulfilled Orders");
    println!("5. Edit Order");
    println!("6. Exit");
    print!("Enter your choice (1-6): ");
    io::stdout().flush().unwrap();
}

fn create_order_cli(inventory: &mut Inventory) {
    println!(" Creating New Order");
    println!("-------------------");

    // Get item type
    println!("Available items:");
    println!("1. Biro");
    println!("2. Stapler");
    println!("3. Books");
    println!("4. Pencil");
    println!("5. Eraser");
    print!("Select item (1-5): ");
    io::stdout().flush().unwrap();

    let item_choice = get_input();
    let item_name = match item_choice.trim() {
        "1" => AllItems::Biro,
        "2" => AllItems::Stapler,
        "3" => AllItems::Books,
        "4" => AllItems::Pencil,
        "5" => AllItems::Eraser,
        _ => {
            println!("❌ Invalid item selection!");
            return;
        }
    };

    // Get quantity
    print!("Enter quantity: ");
    io::stdout().flush().unwrap();
    let quantity_input = get_input();
    let quantity: u32 = match quantity_input.trim().parse() {
        Ok(q) => q,
        Err(_) => {
            println!("Invalid quantity!");
            return;
        }
    };

    // Get supplier
    print!("Enter supplier name: ");
    io::stdout().flush().unwrap();
    let supplier = get_input().trim().to_string();

    // Get status
    println!("Order status:");
    println!("1. Fulfilled");
    println!("2. Pending");
    print!("Select status (1-2): ");
    io::stdout().flush().unwrap();

    let status_choice = get_input();
    let status = match status_choice.trim() {
        "1" => OrderState::Fulfilled,
        "2" => {
            print!("Enter pending amount: ");
            io::stdout().flush().unwrap();
            let pending_input = get_input();
            let pending_amount: u32 = match pending_input.trim().parse() {
                Ok(p) => p,
                Err(_) => {
                    println!("Invalid pending amount!");
                    return;
                }
            };
            OrderState::Pending(pending_amount)
        }
        _ => {
            println!("Invalid status selection!");
            return;
        }
    };

    // Get request ID
    println!("Available request IDs:");
    println!("1. Ayoola");
    println!("2. Nonso");
    println!("3. Femi");
    print!("Select request ID (1-3): ");
    io::stdout().flush().unwrap();

    let id_choice = get_input();
    let request_id = match id_choice.trim() {
        "1" => RequestId::Ayoola,
        "2" => RequestId::Nonso,
        "3" => RequestId::Femi,
        _ => {
            println!("Invalid request ID!");
            return;
        }
    };

    let item = Item {
        name: item_name,
        quantity,
        supplier,
        status,
    };

    inventory.create_order(item, request_id);
    println!("Order created successfully!");
}

fn view_order_cli(inventory: &Inventory) {
    println!("View Order by ID");
    println!("-----------------");

    println!("Available request IDs:");
    println!("1. Ayoola");
    println!("2. Nonso");
    println!("3. Femi");
    print!("Select request ID (1-3): ");
    io::stdout().flush().unwrap();

    let id_choice = get_input();
    let request_id = match id_choice.trim() {
        "1" => RequestId::Ayoola,
        "2" => RequestId::Nonso,
        "3" => RequestId::Femi,
        _ => {
            println!("Invalid request ID!");
            return;
        }
    };

    inventory.view_order_by_id(request_id);
}

fn view_all_orders_cli(inventory: &Inventory) {
    println!("All Orders");
    println!("------------");
    println!("{:#?}", inventory);
}

fn remove_fulfilled_orders_cli(inventory: &mut Inventory) {
    println!("Removing Fulfilled Orders");
    println!("---------------------------");

    inventory.remove_fulfilled_order();
    println!("Fulfilled orders removed successfully!");
}

fn edit_order_cli(inventory: &mut Inventory) {
    println!("Edit Order");
    println!("-----------");

    println!("Available request IDs:");
    println!("1. Ayoola");
    println!("2. Nonso");
    println!("3. Femi");
    print!("Select request ID to edit (1-3): ");
    io::stdout().flush().unwrap();

    let id_choice = get_input();
    let request_id = match id_choice.trim() {
        "1" => RequestId::Ayoola,
        "2" => RequestId::Nonso,
        "3" => RequestId::Femi,
        _ => {
            println!("Invalid request ID!");
            return;
        }
    };

    println!("Enter new item details:");

    // Get new item details
    println!("Available items:");
    println!("1. Biro");
    println!("2. Stapler");
    println!("3. Books");
    println!("4. Pencil");
    println!("5. Eraser");
    print!("Select item (1-5): ");
    io::stdout().flush().unwrap();

    let item_choice = get_input();
    let item_name = match item_choice.trim() {
        "1" => AllItems::Biro,
        "2" => AllItems::Stapler,
        "3" => AllItems::Books,
        "4" => AllItems::Pencil,
        "5" => AllItems::Eraser,
        _ => {
            println!("Invalid item selection!");
            return;
        }
    };

    print!("Enter quantity: ");
    io::stdout().flush().unwrap();
    let quantity_input = get_input();
    let quantity: u32 = match quantity_input.trim().parse() {
        Ok(q) => q,
        Err(_) => {
            println!("Invalid quantity!");
            return;
        }
    };

    print!("Enter supplier name: ");
    io::stdout().flush().unwrap();
    let supplier = get_input().trim().to_string();

    println!("Order status:");
    println!("1. Fulfilled");
    println!("2. Pending");
    print!("Select status (1-2): ");
    io::stdout().flush().unwrap();

    let status_choice = get_input();
    let status = match status_choice.trim() {
        "1" => OrderState::Fulfilled,
        "2" => {
            print!("Enter pending amount: ");
            io::stdout().flush().unwrap();
            let pending_input = get_input();
            let pending_amount: u32 = match pending_input.trim().parse() {
                Ok(p) => p,
                Err(_) => {
                    println!("Invalid pending amount!");
                    return;
                }
            };
            OrderState::Pending(pending_amount)
        }
        _ => {
            println!("Invalid status selection!");
            return;
        }
    };

    let new_item = Item {
        name: item_name,
        quantity,
        supplier,
        status,
    };

    inventory.edit_order(request_id, new_item);
    println!("Order updated successfully!");
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
