use crate::order::Order;
use crate::utils::get_input;
use std::collections::HashMap;

pub fn add_orders(orders: &mut HashMap<u32, Order>, next_id: &mut u32) {
    let item_name = get_input("Enter item name:");
    let item_quantity_input = get_input("Enter quantity:");
    let item_quantity: u32 = match item_quantity_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number. Order not added.");
            return;
        }
    };
    let supplier = get_input("Enter supplier name:");

    let new_order = Order {
        item_name,
        item_quantity,
        supplier,
    };
    orders.insert(*next_id, new_order);
    println!("Order added with ID: {}", *next_id);
    *next_id += 1;
}

pub fn view_orders(orders: &HashMap<u32, Order>) {
    if orders.is_empty() {
        println!("No orders yet.");
    } else {
        println!("\n *Current Orders*");
        for (id, order) in orders {
            println!(
                "Order ID: {}, Item: {}, Quantity: {}, Supplier: {}",
                id, order.item_name, order.item_quantity, order.supplier
            );
        }
    }
}

pub fn remove_orders(orders: &mut HashMap<u32, Order>) {
    if orders.is_empty() {
        println!("No orders to remove.");
        return;
    }

    view_orders(orders);
    let id_input = get_input("Enter the ID of the order to remove:");
    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID.");
            return;
        }
    };

    match orders.remove(&id) {
        Some(_) => println!("Order with ID {} removed.", id),
        None => println!("No order found with ID {}.", id),
    }
}

pub fn edit_orders(orders: &mut HashMap<u32, Order>) {
    if orders.is_empty() {
        println!("No orders to edit.");
        return;
    }

    view_orders(orders);
    let id_input = get_input("Enter the ID of the order to edit:");
    let id: u32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid ID.");
            return;
        }
    };

    match orders.get(&id) {
        Some(existing_order) => {
            println!(
                "Editing Order ID:{}\nItem: {}\nQuantity: {}\nSupplier: {}",
                id, existing_order.item_name, existing_order.item_quantity, existing_order.supplier
            );

            let new_item_name = get_input("Enter new item name (or leave blank):");
            let new_quantity_input = get_input("Enter new quantity (or leave blank):");
            let new_supplier = get_input("Enter new supplier name (or leave blank):");

            let mut updated_order = existing_order.clone();

            if !new_item_name.is_empty() {
                updated_order.item_name = new_item_name;
            }

            if !new_quantity_input.is_empty() {
                match new_quantity_input.trim().parse::<u32>() {
                    Ok(q) => updated_order.item_quantity = q,
                    Err(_) => {
                        println!("Invalid quantity. Edit canceled.");
                        return;
                    }
                }
            }

            if !new_supplier.is_empty() {
                updated_order.supplier = new_supplier;
            }

            let confirm = get_input("Save changes? (yes/no):");
            if confirm.to_lowercase() == "yes" {
                orders.insert(id, updated_order);
                println!("Order updated.");
            } else {
                println!("Edit canceled.");
            }
        }
        None => {
            println!("No order found with ID {}.", id);
        }
    }
}
