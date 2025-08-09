use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use std::io;

#[derive(Clone)]
struct Subscription {
    service_name: String,
    cost: f64,
    renewal_date: DateTime<Utc>,
}

fn add_subscription(subscriptions: &mut HashMap<String, Subscription>) {
    println!("\n--- Add New Subscription ---");

    // Get service name
    println!("Service Name: ");
    let mut service_name = String::new();
    io::stdin()
        .read_line(&mut service_name)
        .expect("Failed to read input");
    let service_name = service_name.trim().to_string();

    // Check if subscription already exists
    if subscriptions.contains_key(&service_name) {
        println!("Subscription for '{}' already exists!", service_name);
        return;
    }

    // Get Service cost
    println!("\nMonthly Cost: $");
    let mut cost_input = String::new();
    io::stdin()
        .read_line(&mut cost_input)
        .expect("Failed to read input");
    let cost: f64 = match cost_input.trim().parse() {
        Ok(c) => c,
        Err(_) => {
            println!("Invalid cost! Please enter a number.");
            return;
        }
    };

    // Get renewal date
    println!("\nRenewal date (YYYY-MM-DD): ");
    let mut date_input = String::new();
    io::stdin()
        .read_line(&mut date_input)
        .expect("Failed to read input");

    // Parse date with UTC
    let renewal_date = match NaiveDate::parse_from_str(date_input.trim(), "%Y-%m-%d") {
        Ok(date) => date.and_hms_opt(0, 0, 0).unwrap().and_utc(),
        Err(_) => {
            println!("Invalid date format! Please use YYYY-MM-DD.");
            return;
        }
    };

    let subscription = Subscription {
        service_name: service_name.clone(),
        cost,
        renewal_date,
    };

    subscriptions.insert(service_name, subscription);
    println!("\nSubscription added successfully!");
}

fn view_subscriptions(subscriptions: &HashMap<String, Subscription>) {
    println!("\n--- All Subscriptions ---");

    if subscriptions.is_empty() {
        println!("No subscriptions found.");
        return;
    }

    let mut subs: Vec<&Subscription> = subscriptions.values().collect();
    subs.sort_by(|a, b| a.service_name.cmp(&b.service_name));

    for (i, sub) in subs.iter().enumerate() {
        let status = if sub.renewal_date < Utc::now() {
            " (EXPIRED)"
        } else {
            ""
        };

        println!(
            "{}. {} - ${:.2}/month - Renews: {}{}",
            i + 1,
            sub.service_name,
            sub.cost,
            sub.renewal_date.format("%Y-%m-%d"),
            status
        );
    }
}

fn remove_expired_subscriptions(subscriptions: &mut HashMap<String, Subscription>) {
    println!("\n--- Remove Expired Subscriptions ---");

    let current_time = Utc::now();
    let expired_services: Vec<String> = subscriptions
        .iter()
        .filter(|(_, sub)| sub.renewal_date < current_time)
        .map(|(key, _)| key.clone())
        .collect();

    if expired_services.is_empty() {
        println!("No expired subscriptions found.");
        return;
    }

    println!("Found {} expired subscription(s):", expired_services.len());
    for service in &expired_services {
        if let Some(sub) = subscriptions.get(service) {
            println!(
                "- {} (expired: {})",
                service,
                sub.renewal_date.format("%Y-%m-%d")
            );
        }
    }

    println!("Remove all expired subscriptions? (y/n): ");
    let mut confirm = String::new();
    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read input");

    if confirm.trim().to_lowercase() == "y" {
        for service in expired_services {
            subscriptions.remove(&service);
        }
        println!("Expired subscriptions removed successfully!");
    } else {
        println!("Removal cancelled.");
    }
}

fn edit_subscription(subscriptions: &mut HashMap<String, Subscription>) {
    println!("\n--- Edit Subscription ---");

    if subscriptions.is_empty() {
        println!("No subscriptions to edit.");
        return;
    }

    // Display available subscriptions
    println!("Available subscriptions:");
    let service_names: Vec<String> = subscriptions.keys().cloned().collect();
    for (i, name) in service_names.iter().enumerate() {
        println!("{}. {}", i + 1, name);
    }

    println!("\nEnter subscription number to edit (or 'c' to cancel): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input.trim().to_lowercase() == "c" {
        println!("Edit cancelled.");
        return;
    }

    let selection: usize = match input.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= service_names.len() => n - 1,
        _ => {
            println!("Invalid selection!");
            return;
        }
    };

    let service_key = &service_names[selection];
    let current_sub = (*subscriptions.get(service_key).unwrap()).clone();

    println!("\nEditing: {}", current_sub.service_name);
    println!("Current details:");
    println!("  Cost: ${:.2}/month", current_sub.cost);
    println!(
        "  Renewal Date: {}",
        current_sub.renewal_date.format("%Y-%m-%d")
    );

    // Create a copy for editing
    let mut edited_sub: Subscription = current_sub.clone();
    let mut changes_made = false;

    // Edit cost
    println!("\n--- Edit Cost ---");
    println!(
        "Current cost: ${:.2} (press Enter to keep, or enter new cost): $",
        edited_sub.cost
    );
    let mut cost_input = String::new();
    io::stdin()
        .read_line(&mut cost_input)
        .expect("Failed to read input");

    if !cost_input.trim().is_empty() {
        match cost_input.trim().parse::<f64>() {
            Ok(new_cost) => {
                edited_sub.cost = new_cost;
                changes_made = true;
                println!("Cost updated to ${:.2}", new_cost);
            }
            Err(_) => {
                println!("Invalid cost format! Keeping current value.");
            }
        }
    }

    // Edit renewal date
    println!("\n--- Edit Renewal Date ---");
    println!(
        "Current date: {} (press Enter to keep, or enter new date YYYY-MM-DD): ",
        edited_sub.renewal_date.format("%Y-%m-%d")
    );
    let mut date_input = String::new();
    io::stdin()
        .read_line(&mut date_input)
        .expect("Failed to read input");

    if !date_input.trim().is_empty() {
        match NaiveDate::parse_from_str(date_input.trim(), "%Y-%m-%d") {
            Ok(date) => {
                edited_sub.renewal_date = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                changes_made = true;
                println!("Renewal date updated to {}", date.format("%Y-%m-%d"));
            }
            Err(_) => {
                println!("Invalid date format! Keeping current value.");
            }
        }
    }

    // Show summary and confirm changes
    if changes_made {
        println!("\n--- Summary of Changes ---");
        println!("Service: {}", edited_sub.service_name);
        println!(
            "Cost: ${:.2}/month (was ${:.2})",
            edited_sub.cost, current_sub.cost
        );
        println!(
            "Renewal: {} (was {})",
            edited_sub.renewal_date.format("%Y-%m-%d"),
            current_sub.renewal_date.format("%Y-%m-%d")
        );

        println!("\nSave changes? (y/n): ");
        let mut confirm = String::new();
        io::stdin()
            .read_line(&mut confirm)
            .expect("Failed to read input");

        if confirm.trim().to_lowercase() == "y" {
            subscriptions.insert(service_key.clone(), edited_sub.clone());
            println!("Changes saved successfully!");
        } else {
            println!("Changes cancelled. Original subscription preserved.");
        }
    } else {
        println!("\nNo changes made.");
    }
}

fn main() {
    let mut subscriptions: HashMap<String, Subscription> = HashMap::new();

    loop {
        println!("\nHello, Customer. What do you want to do today?\n");

        println!(
            "1. Add Subscription\n2. View Subscriptions\n3. Remove Expired Subscriptions\n4. Edit Subscription\n5. Exit\n"
        );
        let mut input = String::new();

        println!("Choose an action (1/2/3/4/5): ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "1" => add_subscription(&mut subscriptions),
            "2" => view_subscriptions(&subscriptions),
            "3" => remove_expired_subscriptions(&mut subscriptions),
            "4" => edit_subscription(&mut subscriptions),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option! Please try again."),
        }
    }
}
