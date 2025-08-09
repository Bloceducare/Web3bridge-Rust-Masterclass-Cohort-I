mod queue;
mod ticket;
mod ui;

use crate::queue::{create_queue, switch_to_hashmap};
use crate::ui::{
    add_ticket, display_menu, edit_ticket, get_user_input, remove_ticket, view_tickets,
};

fn main() {
    let mut queue = create_queue();
    let mut stage = 1;

    println!("Welcome to Customer Support Queue System!");
    println!("Starting with Stage 1 (Add and View)");

    loop {
        display_menu(stage);
        let choice = get_user_input("");

        match choice.as_str() {
            "1" => add_ticket(&mut queue),
            "2" => view_tickets(&mut queue),
            "3" => {
                if stage >= 2 {
                    remove_ticket(&mut queue);
                } else {
                    println!("Invalid choice. Please enter a valid option.");
                }
            }
            "4" => {
                if stage >= 3 {
                    edit_ticket(&mut queue);
                } else {
                    println!("Invalid choice. Please enter a valid option.");
                }
            }
            "0" => {
                println!("Thank you for using Customer Support Queue System!");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option.");
            }
        }

        if stage == 1 {
            stage = 2;
            switch_to_hashmap(&mut queue);
            println!("Advanced to Stage 2 (Remove functionality enabled)!");
        }

        if stage == 2 {
            stage = 3;
            println!("Advanced to Stage 3 (Edit functionality enabled)!");
        }
    }
}
