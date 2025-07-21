//! Integration tests for the Employee Contact Manager

use employee_contact_manager::{Employee, EmployeeManager, EmployeeError, Result, UserInterface};

#[test]
fn test_full_employee_lifecycle() -> Result<()> {
    let mut manager = EmployeeManager::new();
    
    // Test adding employees
    let employee1 = Employee::new(
        "Alice Johnson".to_string(),
        "Engineering".to_string(),
        "555-1234".to_string(),
    )?;
    
    let employee2 = Employee::new(
        "Bob Smith".to_string(),
        "Marketing".to_string(),
        "555-5678".to_string(),
    )?;
    
    manager.add_employee(employee1)?;
    manager.add_employee(employee2)?;
    
    assert_eq!(manager.employee_count(), 2);
    
    // Test retrieving employees
    let alice = manager.get_employee("Alice Johnson")?;
    assert_eq!(alice.department, "Engineering");
    assert_eq!(alice.phone, "555-1234");
    
    // Test updating employee
    manager.update_employee_department("Alice Johnson", "Senior Engineering".to_string())?;
    let updated_alice = manager.get_employee("Alice Johnson")?;
    assert_eq!(updated_alice.department, "Senior Engineering");
    
    // Test removing employee
    let removed = manager.remove_employee("Bob Smith")?;
    assert_eq!(removed.name, "Bob Smith");
    assert_eq!(manager.employee_count(), 1);
    
    // Test error handling
    let result = manager.get_employee("Nonexistent Employee");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), EmployeeError::NotFound("Nonexistent Employee".to_string()));
    
    Ok(())
}

#[test]
fn test_employee_validation() {
    // Test empty name validation
    let result = Employee::new("".to_string(), "Engineering".to_string(), "555-1234".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), EmployeeError::InvalidInput("Name cannot be empty".to_string()));
    
    // Test empty department validation
    let result = Employee::new("John Doe".to_string(), "".to_string(), "555-1234".to_string());
    assert!(result.is_err());
    
    // Test empty phone validation
    let result = Employee::new("John Doe".to_string(), "Engineering".to_string(), "".to_string());
    assert!(result.is_err());
    
    // Test whitespace trimming
    let employee = Employee::new(
        "  John Doe  ".to_string(),
        "  Engineering  ".to_string(),
        "  555-1234  ".to_string(),
    ).unwrap();
    
    assert_eq!(employee.name, "John Doe");
    assert_eq!(employee.department, "Engineering");
    assert_eq!(employee.phone, "555-1234");
}

#[test]
fn test_department_search() -> Result<()> {
    let mut manager = EmployeeManager::new();
    
    // Add employees in different departments
    manager.add_employee(Employee::new("Alice".to_string(), "Engineering".to_string(), "555-1111".to_string())?)?;
    manager.add_employee(Employee::new("Bob".to_string(), "Engineering".to_string(), "555-2222".to_string())?)?;
    manager.add_employee(Employee::new("Charlie".to_string(), "HR".to_string(), "555-3333".to_string())?)?;
    manager.add_employee(Employee::new("Diana".to_string(), "Marketing".to_string(), "555-4444".to_string())?)?;
    
    // Test finding by department
    let engineering = manager.find_by_department("Engineering");
    assert_eq!(engineering.len(), 2);
    
    let hr = manager.find_by_department("HR");
    assert_eq!(hr.len(), 1);
    assert_eq!(hr[0].name, "Charlie");
    
    let marketing = manager.find_by_department("Marketing");
    assert_eq!(marketing.len(), 1);
    assert_eq!(marketing[0].name, "Diana");
    
    // Test case insensitive search
    let engineering_lower = manager.find_by_department("engineering");
    assert_eq!(engineering_lower.len(), 2);
    
    // Test non-existent department
    let sales = manager.find_by_department("Sales");
    assert_eq!(sales.len(), 0);
    
    Ok(())
}

#[test]
fn test_user_interface_basic_operations() -> Result<()> {
    let mut ui = UserInterface::new();
    
    // Test initial state
    assert_eq!(ui.employee_count(), 0);
    
    // Test adding employee through UI
    let employee = Employee::new(
        "Test User".to_string(),
        "Test Department".to_string(),
        "555-0000".to_string(),
    )?;
    
    ui.add_employee(employee)?;
    assert_eq!(ui.employee_count(), 1);
    
    Ok(())
}

#[test]
fn test_error_display() {
    let error1 = EmployeeError::NotFound("John Doe".to_string());
    assert_eq!(format!("{}", error1), "Employee 'John Doe' not found");
    
    let error2 = EmployeeError::InvalidInput("Name is required".to_string());
    assert_eq!(format!("{}", error2), "Invalid input: Name is required");
    
    let error3 = EmployeeError::OperationCancelled;
    assert_eq!(format!("{}", error3), "Operation cancelled by user");
}

#[test]
fn test_employee_display() -> Result<()> {
    let employee = Employee::new(
        "John Doe".to_string(),
        "Engineering".to_string(),
        "555-1234".to_string(),
    )?;
    
    let display = format!("{}", employee);
    assert!(display.contains("John Doe"));
    assert!(display.contains("Engineering"));
    assert!(display.contains("555-1234"));
    assert!(display.contains("Name:"));
    assert!(display.contains("Department:"));
    assert!(display.contains("Phone:"));
    
    Ok(())
}

#[test]
fn test_manager_default() {
    let manager = EmployeeManager::default();
    assert!(manager.is_empty());
    assert_eq!(manager.employee_count(), 0);
}

#[test]
fn test_ui_default() {
    let ui = UserInterface::default();
    assert_eq!(ui.employee_count(), 0);
}