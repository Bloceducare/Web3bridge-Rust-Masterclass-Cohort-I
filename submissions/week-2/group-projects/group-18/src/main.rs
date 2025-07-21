mod models;
mod manager;
mod input;
mod menu;
mod ui;
mod persistence;

use std::thread;
use std::time::Duration;
use customer_feedback_logger::utils::delay_execution;
use manager::FeedbackManager;
use input::get_user_input;
use menu::{add_feedback_menu, remove_feedback_menu, edit_feedback_menu};
use ui::{display_welcome, display_menu, display_goodbye};

fn main() {
    let mut manager = FeedbackManager::new();

    display_welcome();

    loop {
        display_menu();

        let choice: u8;
        loop {
            let _choice = match get_user_input("Enter your choice (1-6): ") {
                Ok(input) => input,
                Err(_) => {
                    println!("Error reading input. Please try again.");
                    delay_execution();
                    continue;
                }
            };

             choice = match _choice.trim().parse() {
                Ok(c) => c,
                Err(_) => {
                    println!("Please enter a valid number (1-6).");
                    delay_execution();
                    continue;
                }
            };
            break;
        }

        match choice {
            1 => add_feedback_menu(&mut manager),
            2 => manager.view_all_feedback(),
            3 => remove_feedback_menu(&mut manager),
            4 => edit_feedback_menu(&mut manager),
            5 => {
                match manager.save_data() {
                    Ok(()) => println!("Data saved successfully!"),
                    Err(e) => println!("Failed to save data: {:?}", e),
                }
            }
            6 => {
                // Final save before exit
                if let Err(e) = manager.save_data() {
                    println!("Warning: Failed to save data before exit: {:?}", e);
                }
                display_goodbye();
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 6."),
        }
        delay_execution();
    }
}
