use crate::queue::{
    CustomerSupportQueue, add_ticket_to_queue, display_tickets, get_ticket_from_queue,
    remove_ticket_from_queue, update_ticket_in_queue,
};
use crate::ticket::{Priority, priority_to_string, string_to_priority};
use std::io::{self, Write};

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_priority_from_user() -> Option<Priority> {
    println!("Priority levels: Low, Medium, High, Critical");
    let priority_input = get_user_input("Enter priority: ");
    string_to_priority(&priority_input)
}

pub fn add_ticket(queue: &mut CustomerSupportQueue) {
    println!("\n=== ADD NEW TICKET ===");

    let customer_name = get_user_input("Enter customer name: ");
    if customer_name.is_empty() {
        println!("Error: Customer name cannot be empty.");
        return;
    }

    let issue_description = get_user_input("Enter issue description: ");
    if issue_description.is_empty() {
        println!("Error: Issue description cannot be empty.");
        return;
    }

    let priority = match get_priority_from_user() {
        Some(p) => p,
        None => {
            println!("Error: Invalid priority level.");
            return;
        }
    };

    let ticket_id = add_ticket_to_queue(queue, customer_name, issue_description, priority);
    println!("Ticket #{} added successfully!", ticket_id);
}

pub fn view_tickets(queue: &CustomerSupportQueue) {
    display_tickets(queue);
}

pub fn remove_ticket(queue: &mut CustomerSupportQueue) {
    println!("\n=== REMOVE TICKET ===");

    let ticket_id_input = get_user_input("Enter ticket ID to remove: ");
    let ticket_id: u32 = match ticket_id_input.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Error: Invalid ticket ID.");
            return;
        }
    };

    if remove_ticket_from_queue(queue, ticket_id) {
        println!("Ticket #{} removed successfully!", ticket_id);
    } else {
        println!("Error: Ticket #{} not found.", ticket_id);
    }
}

pub fn edit_ticket(queue: &mut CustomerSupportQueue) {
    println!("\n=== EDIT TICKET ===");

    let ticket_id_input = get_user_input("Enter ticket ID to edit: ");
    let ticket_id: u32 = match ticket_id_input.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Error: Invalid ticket ID.");
            return;
        }
    };

    if get_ticket_from_queue(queue, ticket_id).is_none() {
        println!("Error: Ticket #{} not found.", ticket_id);
        return;
    }

    println!("Current ticket details:");
    if let Some(ticket) = get_ticket_from_queue(queue, ticket_id) {
        println!("Customer: {}", ticket.customer_name);
        println!("Description: {}", ticket.issue_description);
        println!("Priority: {}", priority_to_string(&ticket.priority));
    }

    println!("\nEnter new values (press Enter to keep current value):");

    let new_customer_name = get_user_input("New customer name: ");
    let new_description = get_user_input("New description: ");
    let new_priority_input = get_user_input("New priority (Low/Medium/High/Critical): ");

    let mut customer_name = None;
    let mut description = None;
    let mut priority = None;

    if !new_customer_name.is_empty() {
        customer_name = Some(new_customer_name);
    }
    if !new_description.is_empty() {
        description = Some(new_description);
    }
    if !new_priority_input.is_empty() {
        priority = string_to_priority(&new_priority_input);
    }

    println!("\nSave changes? (y/n): ");
    let confirm = get_user_input("").to_lowercase();

    if confirm == "y" || confirm == "yes" {
        if update_ticket_in_queue(queue, ticket_id, customer_name, description, priority) {
            println!("Ticket #{} updated successfuly!", ticket_id);
        } else {
            println!("Error: Failed to update ticket.");
        }
    } else {
        println!("Edit cancelled.");
    }
}

pub fn display_menu(stage: u32) {
    println!("\n=== CUSTOMER SUPPORT QUEUE SYSTEM ===");
    println!("Stage {} - Current Features:", stage);
    println!("1. Add Ticket");
    println!("2. View All Tickets");

    if stage >= 2 {
        println!("3. Remove Ticket");
    }

    if stage >= 3 {
        println!("4. Edit Ticket");
    }

    println!("0. Exit");
    println!("Enter your choice: ");
}
