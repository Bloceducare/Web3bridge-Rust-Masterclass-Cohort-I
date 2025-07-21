//! User interface functionality for the Employee Contact Manager

use crate::employee::Employee;
use crate::error::{EmployeeError, Result};
use crate::manager::EmployeeManager;
use std::io::{self, Write};

/// Handles user interface interactions
pub struct UserInterface {
    manager: EmployeeManager,
}

/// Represents the main menu options
#[derive(Debug, PartialEq)]
pub enum MenuOption {
    AddEmployee,
    ViewAllEmployees,
    RemoveEmployee,
    EditEmployee,
    Exit,
    Invalid,
}

impl From<&str> for MenuOption {
    fn from(input: &str) -> Self {
        match input {
            "1" => MenuOption::AddEmployee,
            "2" => MenuOption::ViewAllEmployees,
            "3" => MenuOption::RemoveEmployee,
            "4" => MenuOption::EditEmployee,
            "5" => MenuOption::Exit,
            _ => MenuOption::Invalid,
        }
    }
}

impl UserInterface {
    /// Creates a new UserInterface instance
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_contact_manager::UserInterface;
    ///
    /// let ui = UserInterface::new();
    /// ```
    pub fn new() -> Self {
        UserInterface {
            manager: EmployeeManager::new(),
        }
    }

    /// Displays the main menu and returns the user's choice
    fn display_menu(&self) -> MenuOption {
        println!("\n=== Employee Contact Manager ===");
        println!("1. Add Employee");
        println!("2. View All Employees");
        println!("3. Remove Employee");
        println!("4. Edit Employee");
        println!("5. Exit");
        
        let choice = self.get_input("Enter your choice (1-5): ");
        MenuOption::from(choice.as_str())
    }

    /// Gets input from the user with a prompt
    fn get_input(&self, prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// Gets a yes/no confirmation from the user
    fn get_confirmation(&self, prompt: &str) -> bool {
        let input = self.get_input(&format!("{} (y/n): ", prompt));
        matches!(input.to_lowercase().as_str(), "y" | "yes")
    }

    /// Handles adding a new employee
    fn handle_add_employee(&mut self) -> Result<()> {
        println!("\n--- Add New Employee ---");
        
        let name = self.get_input("Enter employee name: ");
        let department = self.get_input("Enter department: ");
        let phone = self.get_input("Enter phone number: ");
        
        match Employee::new(name, department, phone) {
            Ok(employee) => {
                self.manager.add_employee(employee)?;
                println!("✓ Employee added successfully!");
            }
            Err(e) => {
                println!("✗ Error: {}", e);
                return Err(e);
            }
        }
        
        Ok(())
    }

    /// Handles viewing all employees
    fn handle_view_employees(&self) {
        println!("\n--- All Employees ---");
        
        if self.manager.is_empty() {
            println!("No employees found.");
            return;
        }

        for (index, employee) in self.manager.employees().enumerate() {
            println!("\n{}. {}", index + 1, employee);
            println!("------------------------");
        }
        
        println!("\nTotal employees: {}", self.manager.employee_count());
    }

    /// Handles removing an employee
    fn handle_remove_employee(&mut self) -> Result<()> {
        if self.manager.is_empty() {
            println!("No employees to remove.");
            return Ok(());
        }

        println!("\n--- Remove Employee ---");
        let name = self.get_input("Enter employee name to remove: ");
        
        match self.manager.get_employee(&name) {
            Ok(employee) => {
                println!("\nEmployee found:");
                println!("{}", employee);
                
                if self.get_confirmation("Are you sure you want to remove this employee?") {
                    self.manager.remove_employee(&name)?;
                    println!("✓ Employee '{}' removed successfully!", name);
                } else {
                    println!("Operation cancelled.");
                }
            }
            Err(e) => {
                println!("✗ {}", e);
                return Err(e);
            }
        }
        
        Ok(())
    }

    /// Handles editing an employee
    fn handle_edit_employee(&mut self) -> Result<()> {
        if self.manager.is_empty() {
            println!("No employees to edit.");
            return Ok(());
        }

        println!("\n--- Edit Employee ---");
        let name = self.get_input("Enter employee name to edit: ");
        
        // Check if employee exists
        match self.manager.get_employee(&name) {
            Ok(employee) => {
                println!("\nCurrent employee information:");
                println!("{}", employee);
                
                // Store original values for potential rollback
                let original_department = employee.department.clone();
                let original_phone = employee.phone.clone();
                
                // Get new values
                println!("\nEnter new information (press Enter to keep current value):");
                
                let new_department = self.get_input(&format!("Department [{}]: ", employee.department));
                let new_phone = self.get_input(&format!("Phone [{}]: ", employee.phone));
                
                // Track if any changes were made
                let mut changes_made = false;
                
                // Update department if provided
                if !new_department.is_empty() {
                    match self.manager.update_employee_department(&name, new_department) {
                        Ok(()) => changes_made = true,
                        Err(e) => {
                            println!("✗ Error updating department: {}", e);
                            return Err(e);
                        }
                    }
                }
                
                // Update phone if provided
                if !new_phone.is_empty() {
                    match self.manager.update_employee_phone(&name, new_phone) {
                        Ok(()) => changes_made = true,
                        Err(e) => {
                            println!("✗ Error updating phone: {}", e);
                            // Rollback department change if it was made
                            if changes_made {
                                let _ = self.manager.update_employee_department(&name, original_department);
                            }
                            return Err(e);
                        }
                    }
                }
                
                if !changes_made {
                    println!("No changes made.");
                    return Ok(());
                }
                
                // Show updated information and confirm
                let updated_employee = self.manager.get_employee(&name)?;
                println!("\nUpdated employee information:");
                println!("{}", updated_employee);
                
                if self.get_confirmation("Confirm these changes?") {
                    println!("✓ Employee updated successfully!");
                } else {
                    // Rollback changes
                    let _ = self.manager.update_employee_department(&name, original_department);
                    let _ = self.manager.update_employee_phone(&name, original_phone);
                    println!("Changes cancelled - employee information restored.");
                    return Err(EmployeeError::OperationCancelled);
                }
            }
            Err(e) => {
                println!("✗ {}", e);
                return Err(e);
            }
        }
        
        Ok(())
    }

    /// Runs the main application loop
    ///
    /// This method displays the menu and handles user input until the user chooses to exit.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use employee_contact_manager::UserInterface;
    ///
    /// let mut ui = UserInterface::new();
    /// ui.run();
    /// ```
    pub fn run(&mut self) {
        println!("Welcome to Employee Contact Manager!");
        
        loop {
            match self.display_menu() {
                MenuOption::AddEmployee => {
                    if let Err(e) = self.handle_add_employee() {
                        if !matches!(e, EmployeeError::InvalidInput(_)) {
                            eprintln!("Unexpected error: {}", e);
                        }
                    }
                }
                MenuOption::ViewAllEmployees => {
                    self.handle_view_employees();
                }
                MenuOption::RemoveEmployee => {
                    if let Err(e) = self.handle_remove_employee() {
                        if !matches!(e, EmployeeError::NotFound(_)) {
                            eprintln!("Unexpected error: {}", e);
                        }
                    }
                }
                MenuOption::EditEmployee => {
                    if let Err(e) = self.handle_edit_employee() {
                        if !matches!(e, EmployeeError::NotFound(_) | EmployeeError::OperationCancelled | EmployeeError::InvalidInput(_)) {
                            eprintln!("Unexpected error: {}", e);
                        }
                    }
                }
                MenuOption::Exit => {
                    println!("Thank you for using Employee Contact Manager. Goodbye!");
                    break;
                }
                MenuOption::Invalid => {
                    println!("Invalid choice. Please try again.");
                }
            }
        }
    }

    /// Returns the number of employees currently managed
    ///
    /// This method is primarily useful for testing
    pub fn employee_count(&self) -> usize {
        self.manager.employee_count()
    }

    /// Adds an employee directly (useful for testing)
    ///
    /// # Arguments
    ///
    /// * `employee` - The employee to add
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if the operation fails
    pub fn add_employee(&mut self, employee: Employee) -> Result<()> {
        self.manager.add_employee(employee)
    }
}

impl Default for UserInterface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_option_from_string() {
        assert_eq!(MenuOption::from("1"), MenuOption::AddEmployee);
        assert_eq!(MenuOption::from("2"), MenuOption::ViewAllEmployees);
        assert_eq!(MenuOption::from("3"), MenuOption::RemoveEmployee);
        assert_eq!(MenuOption::from("4"), MenuOption::EditEmployee);
        assert_eq!(MenuOption::from("5"), MenuOption::Exit);
        assert_eq!(MenuOption::from("6"), MenuOption::Invalid);
        assert_eq!(MenuOption::from("invalid"), MenuOption::Invalid);
    }

    #[test]
    fn test_user_interface_creation() {
        let ui = UserInterface::new();
        assert_eq!(ui.employee_count(), 0);
    }

    #[test]
    fn test_add_employee_to_ui() {
        let mut ui = UserInterface::new();
        let employee = Employee::new(
            "Test Employee".to_string(),
            "Test Department".to_string(),
            "555-0000".to_string(),
        ).unwrap();

        ui.add_employee(employee).unwrap();
        assert_eq!(ui.employee_count(), 1);
    }
}