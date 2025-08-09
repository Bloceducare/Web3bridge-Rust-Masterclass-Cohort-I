use crate::ticket::{Priority, Ticket, create_ticket, priority_to_string};
use std::collections::HashMap;

pub struct CustomerSupportQueue {
    pub tickets: Vec<Ticket>,
    pub ticket_map: HashMap<u32, Ticket>,
    pub next_id: u32,
    pub use_hashmap: bool,
}

pub fn create_queue() -> CustomerSupportQueue {
    CustomerSupportQueue {
        tickets: Vec::new(),
        ticket_map: HashMap::new(),
        next_id: 1,
        use_hashmap: false,
    }
}

pub fn add_ticket_to_queue(
    queue: &mut CustomerSupportQueue,
    customer_name: String,
    issue_description: String,
    priority: Priority,
) -> u32 {
    let ticket = create_ticket(queue.next_id, customer_name, issue_description, priority);
    let id = queue.next_id;

    if queue.use_hashmap {
        queue.ticket_map.insert(id, ticket);
    } else {
        queue.tickets.push(ticket);
    }

    queue.next_id += 1;
    id
}

pub fn display_tickets(queue: &CustomerSupportQueue) {
    if queue.tickets.is_empty() && queue.ticket_map.is_empty() {
        println!("No tickets found.");
        return;
    }

    println!("\n=== CUSTOMER SUPPORT TICKETS ===");
    println!(
        "{:<5} {:<20} {:<50} {:<10} {:<10}",
        "ID", "Customer", "Description", "Priority", "Status"
    );
    println!("{:-<100}", "");

    if queue.use_hashmap {
        for ticket in queue.ticket_map.values() {
            println!(
                "{:<5} {:<20} {:<50} {:<10} {:<10}",
                ticket.id,
                ticket.customer_name,
                if ticket.issue_description.len() > 47 {
                    format!("{}...", &ticket.issue_description[..47])
                } else {
                    ticket.issue_description.clone()
                },
                priority_to_string(&ticket.priority),
                ticket.status
            );
        }
    } else {
        for ticket in &queue.tickets {
            println!(
                "{:<5} {:<20} {:<50} {:<10} {:<10}",
                ticket.id,
                ticket.customer_name,
                if ticket.issue_description.len() > 47 {
                    format!("{}...", &ticket.issue_description[..47])
                } else {
                    ticket.issue_description.clone()
                },
                priority_to_string(&ticket.priority),
                ticket.status
            );
        }
    }
    println!("{:-<100}", "");
}

pub fn remove_ticket_from_queue(queue: &mut CustomerSupportQueue, ticket_id: u32) -> bool {
    if queue.use_hashmap {
        queue.ticket_map.remove(&ticket_id).is_some()
    } else {
        if let Some(index) = queue.tickets.iter().position(|t| t.id == ticket_id) {
            queue.tickets.remove(index);
            true
        } else {
            false
        }
    }
}

pub fn get_ticket_from_queue(queue: &CustomerSupportQueue, ticket_id: u32) -> Option<&Ticket> {
    if queue.use_hashmap {
        queue.ticket_map.get(&ticket_id)
    } else {
        queue.tickets.iter().find(|t| t.id == ticket_id)
    }
}

pub fn update_ticket_in_queue(
    queue: &mut CustomerSupportQueue,
    ticket_id: u32,
    customer_name: Option<String>,
    issue_description: Option<String>,
    priority: Option<Priority>,
) -> bool {
    if queue.use_hashmap {
        if let Some(ticket) = queue.ticket_map.get_mut(&ticket_id) {
            if let Some(name) = customer_name {
                ticket.customer_name = name;
            }
            if let Some(desc) = issue_description {
                ticket.issue_description = desc;
            }
            if let Some(pri) = priority {
                ticket.priority = pri;
            }
            true
        } else {
            false
        }
    } else {
        if let Some(ticket) = queue.tickets.iter_mut().find(|t| t.id == ticket_id) {
            if let Some(name) = customer_name {
                ticket.customer_name = name;
            }
            if let Some(desc) = issue_description {
                ticket.issue_description = desc;
            }
            if let Some(pri) = priority {
                ticket.priority = pri;
            }
            true
        } else {
            false
        }
    }
}

pub fn switch_to_hashmap(queue: &mut CustomerSupportQueue) {
    if !queue.use_hashmap {
        for ticket in queue.tickets.drain(..) {
            queue.ticket_map.insert(ticket.id, ticket);
        }
        queue.use_hashmap = true;
        println!("Switched to HashMap for efficient lookups!");
    }
}
