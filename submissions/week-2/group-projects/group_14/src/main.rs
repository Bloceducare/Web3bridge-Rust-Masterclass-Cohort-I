mod order;
mod order_actions;
mod utils;

use crate::order::Order;
use crate::utils::get_input;
use crate::order_actions::{add_orders, edit_orders, remove_orders, view_orders};
use std::collections::HashMap;

fn main() {
    let mut orders: HashMap<u32, Order> = HashMap::new();
    let mut next_id: u32 = 1;

    loop {
        println!("\n *Supply Order Tracker*");
        println!("1. Add Order");
        println!("2. View Orders");
        println!("3. Remove Orders");
        println!("4. Edit Orders");
        println!("5. Quit");

        // calls the helper function to get user input/what they want to do next
        let order = get_input("What would you like to do next?:");

        // match statement to check what user would like to do and run the right function. choosing any numbeer from 1 - 5 runs the function. if the user enters anything else, it will print an error message.
        match order.trim() {
            "1" => {
                add_orders(&mut orders, &mut next_id);
            }
            "2" => {
                view_orders(&orders);
            }
            "3" => {
                remove_orders(&mut orders);
            }
            "4" => {
                edit_orders(&mut orders);
            }
            "5" => {
                println!("Thank you for your order!");
                break;
            }
            _ => println!("Invalid order, try again!"),
        }
    }
}
