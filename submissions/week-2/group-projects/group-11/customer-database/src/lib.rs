use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
}

impl Customer {
    pub fn new(id: u32, name: String, email: String, phone: String) -> Self {
        Self { id, name, email, phone }
    }
}

#[derive(Debug)]
pub enum MenuOption {
    Add,
    View,
    ViewAll,
    Remove,
    Edit,
    Exit,
    NotFound,
}


pub fn get_input(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|_| "Failed to display prompt".to_string())?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read input".to_string())?;
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        Err("Input cannot be empty".to_string())
    } else {
        Ok(trimmed_input.to_string())
    }
}

pub fn parse_id_input(prompt: &str) -> Result<u32, String> {
    let input = get_input(prompt)?;
    input.parse::<u32>().map_err(|_| "ID must be a number.".to_string())
}

fn get_valid_email(customers: &HashMap<u32, Customer>) -> Result<String, String> {
    loop {
        match get_input("Enter email: ") {
            Ok(email) => {
                if !email.contains('@') {
                    println!("Error: Email must contain '@' symbol. Please try again.");
                    continue;
                }
                
                let mut email_exists = false;
                for customer in customers.values() {
                    if customer.email == email {
                        email_exists = true;
                        break;
                    }
                }
                
                if email_exists {
                    println!("Error: Email is already registered to another customer. Please try again.");
                    continue;
                }
                
                return Ok(email);
            },
            Err(e) => {
                println!("Error: {}. Please try again.", e);
                continue;
            }
        }
    }
}

/// Stage 1
pub fn add_customer(customers: &mut HashMap<u32, Customer>, next_id: &mut u32) -> Result<(), String> {
    let name = get_input("Enter name: ")?;
    let email = get_valid_email(customers)?;
    let phone = get_input("Enter phone: ")?;

    let customer = Customer::new(*next_id, name, email, phone);
    customers.insert(*next_id, customer);
    println!("Customer added with ID: {}", *next_id);
    *next_id += 1;
    Ok(())
}

pub fn view_customer(customers: &HashMap<u32, Customer>) {
    println!("\n--- View Customer ---");
    
    if customers.is_empty() {
        println!("No customers found.");
        return;
    } else {
        match parse_id_input("Enter customer ID: ") {
            Ok(id) => match customers.get(&id) {
                Some(customer) => println!("ID: {} | Name: {} | Email: {} | Phone: {}", id, customer.name, customer.email, customer .phone),
                None => println!("Customer with ID {} not found.", id),
            },
            Err(e) => println!("Error: {}", e),
        }
    }    
}

fn view_customers(customers: &HashMap<u32, Customer>) {
    if customers.is_empty() {
        println!("No customers found.");
    } else {
        for (id, customer) in customers {
            println!("ID: {} | Name: {} | Email: {} | Phone: {}", id, customer.name, customer.email, customer .phone);
        }
    }
}

/// Stage 2
fn remove_customer(customers: &mut HashMap<u32, Customer>) {
    println!("\n--- Remove Customer ---");

    if customers.is_empty() {
        println!("No customers to remove.");
        return;
    } else {

    // Show all customers first
    view_customers(customers);

    match parse_id_input("Enter customer ID to remove: ") {
        Ok(id) => {
            if let Some(customer) = customers.get(&id) {
                println!("Customer to remove:");
                println!("ID: {} | Name: {} | Email: {} | Phone: {}", 
                        customer.id, customer.name, customer.email, customer.phone);
                
                // Ask for confirmation
                match get_input("Are you sure you want to remove this customer? (y/n): ") {
                    Ok(confirmation) => {
                        if confirmation.to_lowercase() == "y" || confirmation.to_lowercase() == "yes" {
                            if let Some(removed) = customers.remove(&id) {
                                println!("Removed customer: {} with ID {}", removed.name, removed.id);
                            }
                        } else {
                            println!("Remove operation cancelled.");
                        }
                    },
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Customer with ID {} not found.", id);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
    }
}
/// Olumide End 
/// Stage 3
fn edit_customer(customers: &mut HashMap<u32, Customer>) {
    println!("\n--- Edit Customer ---");

    if customers.is_empty() {
        println!("No customers to edit.");
        return;
    } else {
    
    // Show all customers first
    view_customers(customers);

    match parse_id_input("Enter customer ID to edit: ") {
        Ok(id) => {
            if let Some(customer) = customers.get_mut(&id) {
                println!("Customer to edit:");
                println!("ID: {} | Name: {} | Email: {} | Phone: {}", 
                        customer.id, customer.name, customer.email, customer.phone);
                
                // Store original values for comparison
                let original_name = &customer.name;
                let original_email = &customer.email;
                let original_phone = &customer.phone;

                println!("Editing customer: {:?}", customer);

                // Collect changes in temporary variables
                let mut new_name = None;
                let mut new_email = None;
                let mut new_phone = None;

                if let Ok(name) = get_input("Enter new name or leave blank to keep current: ") {
                    if !name.is_empty() {
                        new_name = Some(name);
                    }
                }

                if let Ok(email) = get_input("Enter new email or leave blank to keep current: ") {
                    if !email.is_empty() {
                        new_email = Some(email);
                    }
                }

                if let Ok(phone) = get_input("Enter new phone or leave blank to keep current: ") {
                    if !phone.is_empty() {
                        new_phone = Some(phone);
                    }
                }

                // Output the changes
                println!("\n--- Review Changes ---");
                println!("Original:");
                println!("ID: {} | Name: {} | Email: {} | Phone: {}", 
                        customer.id, original_name, original_email, original_phone);
                println!("Updated:");
                println!("ID: {} | Name: {} | Email: {} | Phone: {}", 
                        customer.id, 
                        new_name.as_ref().unwrap_or(original_name),
                        new_email.as_ref().unwrap_or(original_email),
                        new_phone.as_ref().unwrap_or(original_phone));
                
                // Ask for confirmation after editing
                match get_input("Save these changes? (y/n): ") {
                    Ok(confirmation) => {
                        if confirmation.to_lowercase() == "y" || confirmation.to_lowercase() == "yes" {
                            // Apply changes only if user confirms
                            if let Some(name) = new_name {
                                customer.name = name;
                            }
                            if let Some(email) = new_email {
                                customer.email = email;
                            }
                            if let Some(phone) = new_phone {
                                customer.phone = phone;
                            }
                            println!("Customer Details updated successfully!");
                        } else {
                            println!("Customer Details not updated.");
                        }
                    },
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Customer with ID {} not found.", id);
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}
}


pub fn get_menu_choice() -> Result<MenuOption, String> {
    println!("\n--- Customer Database ---");
    println!("1. Add Customer");
    println!("2. View Customer");
    println!("3. View All Customers");
    println!("4. Remove Customer");
    println!("5. Edit Customer");
    println!("6. Exit");
    print!("Choose option: ");
    io::stdout().flush().map_err(|_| "Failed to display menu".to_string())?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).map_err(|_| "Failed to read input".to_string())?;
    match choice.trim() {
        "1" => Ok(MenuOption::Add),
        "2" => Ok(MenuOption::View),
        "3" => Ok(MenuOption::ViewAll),
        "4" => Ok(MenuOption::Remove),
        "5" => Ok(MenuOption::Edit),
        "6" => Ok(MenuOption::Exit),
        _ => Ok(MenuOption::NotFound),
    }
}


pub fn run_program() {
    println!("Welcome to Group 11's Customer Database!");
    let mut customers: HashMap<u32, Customer> = HashMap::new();
    let mut next_id: u32 = 1;

    loop {
        match get_menu_choice() {
            Ok(MenuOption::Add) => {
                if let Err(e) = add_customer(&mut customers, &mut next_id) {
                    eprintln!("Error adding customer: {}", e);
                }
            },
            Ok(MenuOption::View) => view_customer(&customers),
            Ok(MenuOption::ViewAll) => view_customers(&customers),
            Ok(MenuOption::Remove) => remove_customer(&mut customers),
            Ok(MenuOption::Edit) => edit_customer(&mut customers),
            Ok(MenuOption::Exit) => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid selection. Please try again."),
        }
    }
}

/// Bolarinwa End



