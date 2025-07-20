use access_control::state::{EmployeeInfo, Employee, EmployeeStatus, EmployeeType, AccessControl};

fn main() -> Result<(), String> {
    let mut staff_access_control = EmployeeInfo::new();

    println!("🏢 Web3Bridge Garage Access Control System");
    println!("{}", "=".repeat(50));

    // Add employees with different roles
    let alice_id = staff_access_control.add_employee("Alice".to_string(), EmployeeType::Media, EmployeeStatus::Employed);
    let bob_id = staff_access_control.add_employee("Bob".to_string(), EmployeeType::IT, EmployeeStatus::Employed);
    let charlie_id = staff_access_control.add_employee("Charlie".to_string(), EmployeeType::Manager, EmployeeStatus::Employed);
    let david_id = staff_access_control.add_employee("David".to_string(), EmployeeType::SocialMedia, EmployeeStatus::Employed);
    let eve_id = staff_access_control.add_employee("Eve".to_string(), EmployeeType::TechnicianSupervisor, EmployeeStatus::Employed);
    let frank_id = staff_access_control.add_employee("Frank".to_string(), EmployeeType::KitchenStaff, EmployeeStatus::Employed);

    // Check access for all employees using the ? operator function
    println!("\n📋 Initial Access Status:");
    staff_access_control.print_access_status(alice_id)?;
    staff_access_control.print_access_status(bob_id)?;
    staff_access_control.print_access_status(charlie_id)?;
    staff_access_control.print_access_status(david_id)?;
    staff_access_control.print_access_status(eve_id)?;
    staff_access_control.print_access_status(frank_id)?;

    // Generate access keys for authorized employees
    println!("\n🔑 Generating Access Keys:");
    let alice_key = match staff_access_control.generate_access_key(alice_id) {
        Ok(key) => {
            println!("✅ Access key generated for Alice: {}", key);
            Some(key)
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            None
        }
    };

    match staff_access_control.generate_access_key(david_id) {
        Ok(key) => println!("✅ Access key generated for David: {}", key),
        Err(e) => println!("❌ Error: {}", e),
    }

    // **DEMONSTRATE KEY-BASED ACCESS**
    println!("\n🚪 Testing Key-Based Access:");
    
    if let Some(key) = &alice_key {
        // Alice uses her key to gain access
        match staff_access_control.grant_access_with_key(key) {
            Ok(message) => println!("{}", message),
            Err(e) => println!("❌ {}", e),
        }
    }

    // Test with invalid key
    println!("\n🔍 Testing Invalid Key:");
    match staff_access_control.grant_access_with_key("invalid-key-12345") {
        Ok(message) => println!("{}", message),
        Err(e) => println!("❌ {}", e),
    }

    // Generate key for Charlie and test it
    let charlie_key = staff_access_control.generate_access_key(charlie_id).unwrap();
    println!("\n🔑 Charlie's key generated: {}", charlie_key);
    
    match staff_access_control.grant_access_with_key(&charlie_key) {
        Ok(message) => println!("{}", message),
        Err(e) => println!("❌ {}", e),
    }

    // Terminate an employee and check access
    println!("\n🚫 Terminating Bob...");
    
    // First generate a key for Bob
    let bob_key = staff_access_control.generate_access_key(bob_id).unwrap();
    println!("Bob's key before termination: {}", bob_key);
    
    // Bob uses his key successfully
    println!("\n🚪 Bob tries to access before termination:");
    staff_access_control.validate_and_print_access(&bob_key).unwrap();
    
    // Now terminate Bob
    staff_access_control.terminate_employee(bob_id)?;
    staff_access_control.print_access_status(bob_id)?;

    // Try to use Bob's key after termination
    println!("\n🚪 Bob tries to access after termination:");
    match staff_access_control.grant_access_with_key(&bob_key) {
        Ok(message) => println!("{}", message),
        Err(e) => println!("❌ {}", e),
    }

    // Try to generate access key for terminated employee
    println!("\n🔑 Attempting to generate new access key for terminated employee:");
    match staff_access_control.generate_access_key(bob_id) {
        Ok(key) => println!("✅ Access key for Bob: {}", key),
        Err(e) => println!("❌ Error: {}", e),
    }

    // Demonstrate key management features
    println!("\n📋 Active Access Keys:");
    let active_keys = staff_access_control.list_active_keys();
    for (key, emp_id, name) in active_keys {
        println!("🔑 {}: {}...{} (Employee: {})", 
                name, &key[..8], &key[key.len()-4..], emp_id);
    }

    // Revoke Alice's key
    if let Some(key) = &alice_key {
        println!("\n🔐 Revoking Alice's access key...");
        staff_access_control.revoke_access_key(key)?;
        
        // Try to use revoked key
        println!("🚪 Alice tries to use revoked key:");
        match staff_access_control.grant_access_with_key(key) {
            Ok(message) => println!("{}", message),
            Err(e) => println!("❌ {}", e),
        }
    }

    // Update an employee's role
    println!("\n🔄 Promoting David to Manager...");
    staff_access_control.update_employee(david_id, "David (Manager)".to_string(), EmployeeType::Manager)?;
    staff_access_control.print_access_status(david_id)?;

    // Generate access key for newly promoted employee
    match staff_access_control.generate_access_key(david_id) {
        Ok(key) => println!("✅ New access key for David: {}", key),
        Err(e) => println!("❌ Error: {}", e),
    }

    // List all employees
    println!("\n👥 All Employees:");
    for employee in staff_access_control.get_all_employees() {
        let access_symbol = match employee.access {
            AccessControl::Granted => "✅",
            AccessControl::Denied => "❌",
        };
        println!("{} {} (ID: {}) - {:?} - {:?}", 
                access_symbol, employee.name, employee.id, employee.employee_type, employee.status);
    }

    println!("\n✨ System demonstration completed successfully!");
    Ok(())
}