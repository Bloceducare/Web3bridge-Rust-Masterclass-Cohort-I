use expense_report::errors::{EditError, RemoveError};
use expense_report::manager::ExpenseManager;
use expense_report::types::ExpenseStatus;
use std::io::{self, Write};

fn main() {
    run_program();
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn get_confirmation(prompt: &str) -> bool {
    loop {
        let input = get_input(&format!("{} (y/n): ", prompt));
        match input.to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}

fn add_expense_menu(manager: &mut ExpenseManager) {
    println!("Add New Expense");

    let name = get_input("Enter expense name: ");
    if name.is_empty() {
        println!("Error: Expense name cannot be empty");
        return;
    }

    let category = get_input("Enter expense category: ");
    if category.is_empty() {
        println!("Error: Category cannot be empty");
        return;
    }

    let amount = get_input("Enter expense amount: ");
    if amount.is_empty() {
        println!("Error: Amount cannot be empty");
        return;
    }

    match manager.add_expense(name, category, amount) {
        Ok(id) => println!("Expense added successfully with ID: {}", id),
        Err(e) => println!("Error adding expense: {}", e),
    }
}

fn view_expenses_menu(manager: &ExpenseManager) {
    println!("Your Expenses");

    let expenses = manager.view_expenses();

    if expenses.is_empty() {
        println!("No expenses found. Add some expenses first");
        return;
    }

    for expense in expenses {
        let status_str = match expense.status {
            ExpenseStatus::Pending => "Pending",
            ExpenseStatus::Approved => "Approved",
            ExpenseStatus::Rejected => "Rejected",
        };

        println!(
            "ID:{}, Name: {}, Category: {}, Amount: {}, Status: {}",
            expense.id, expense.name, expense.category, expense.amount, status_str
        );
    }
}

fn remove_expense_menu(manager: &mut ExpenseManager) {
    println!("Remove Expense");

    if manager.view_expenses().is_empty() {
        println!("No expenses to remove");
        return;
    }

    view_expenses_menu(manager);

    let id_input = get_input("Enter the ID of the expense to remove: ");
    let id: u32 = match id_input.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid ID number input");
            return;
        }
    };

    if let Some(expense) = manager.view_expenses().iter().find(|e| e.id == id) {
        println!("Expense to remove:");
        println!("ID: {}", expense.id);
        println!("Name: {}", expense.name);
        println!("Category: {}", expense.category);
        println!("Amount: {}", expense.amount);
        println!("Status: {:?}", expense.status);

        if !get_confirmation("Are you sure you want to remove this expense?") {
            println!("Removal cancelled.");
            return;
        }
    }

    match manager.remove_expense(id) {
        Ok(()) => println!("Expense removed successfully"),
        Err(RemoveError::NotFound) => println!("Expense with ID {} not found", id),
        Err(RemoveError::CannotRemovePending) => {
            println!("Cannot remove pending expenses. Please approve or reject the expense first.");
        }
    }
}

fn edit_expense_menu(manager: &mut ExpenseManager) {
    println!("Edit Expense");

    if manager.view_expenses().is_empty() {
        println!("No expenses to edit");
        return;
    }

    view_expenses_menu(manager);

    let id_input = get_input("Enter the ID of the expense to edit: ");
    let id: u32 = match id_input.parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid ID");
            return;
        }
    };

    let current_expense = match manager.view_expenses().iter().find(|e| e.id == id) {
        Some(expense) => (*expense).clone(),
        None => {
            println!("Expense with ID {} not found", id);
            return;
        }
    };

    println!("Current expense details:");
    println!("Name: {}", current_expense.name);
    println!("Category: {}", current_expense.category);
    println!("Amount: {}", current_expense.amount);
    println!("Status: {:?}", current_expense.status);

    println!("Enter new values (press Enter to keep current value):");

    let new_name = get_input(&format!("New name [{}]: ", current_expense.name));
    let new_name = if new_name.is_empty() { None } else { Some(new_name) };

    let new_category = get_input(&format!("New category [{}]: ", current_expense.category));
    let new_category = if new_category.is_empty() { None } else { Some(new_category) };

    let new_amount = get_input(&format!("New amount [{}]: ", current_expense.amount));
    let new_amount = if new_amount.is_empty() { None } else { Some(new_amount) };

    println!("Available statuses: pending, approved, rejected");
    let new_status = get_input(&format!("New status [{:?}]: ", current_expense.status));
    let new_status = if new_status.is_empty() { None } else { Some(new_status) };

    println!("=== Summary of Changes ===");
    if let Some(ref name) = new_name {
        println!("Name: {} -> {}", current_expense.name, name);
    }
    if let Some(ref category) = new_category {
        println!("Category: {} -> {}", current_expense.category, category);
    }
    if let Some(ref amount) = new_amount {
        println!("Amount: {} -> {}", current_expense.amount, amount);
    }
    if let Some(ref status) = new_status {
        println!("Status: {:?} -> {}", current_expense.status, status);
    }

    if new_name.is_none() && new_category.is_none() && new_amount.is_none() && new_status.is_none() {
        println!("No changes to make.");
        return;
    }

    let confirm = get_confirmation("Save these changes?");

    match manager.edit_expense(id, new_name, new_amount, new_category, new_status, confirm) {
        Ok(()) => println!("Expense updated successfully"),
        Err(EditError::NotFound) => println!("Expense not found"),
        Err(EditError::InvalidAmount) => println!("Invalid amount format"),
        Err(EditError::InvalidStatus) => println!("Invalid status! Use: pending, approved, or rejected"),
        Err(EditError::Cancelled) => println!("Edit cancelled."),
    }
}

fn run_program() {
    let mut manager = ExpenseManager::new();

    println!("Expense Manager");

    loop {
        println!("==========================================================");
        println!("MENU");
        println!("==========================================================");
        println!("1. Add an expense");
        println!("2. View all expenses");
        println!("3. Remove an expense");
        println!("4. Edit an expense");
        println!("5. Exit");
        println!("==========================================================");

        let choice = get_input("Enter your choice (1-5): ");

        match choice.as_str() {
            "1" => add_expense_menu(&mut manager),
            "2" => view_expenses_menu(&manager),
            "3" => remove_expense_menu(&mut manager),
            "4" => edit_expense_menu(&mut manager),
            "5" => {
                println!("Exiting program");
                break;
            }
            _ => {
                println!("Invalid choice! Please enter a number between 1 and 5.");
            }
        }
    }
}
