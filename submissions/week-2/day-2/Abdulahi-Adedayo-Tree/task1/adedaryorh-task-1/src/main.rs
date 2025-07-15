// src/main.rs
use adedaryorh_task_1::{ClassManager, StudentStatus};

fn main() {
    // Example usage
    let mut manager = ClassManager::new();
    
    // Register students
    let alice_id = manager.register_student(
        "Alice Johnson".to_string(),
        85.5,
        StudentStatus::Active,
    );
    
    let bob_id = manager.register_student(
        "Bob Smith".to_string(),
        92.0,
        StudentStatus::Active,
    );
    
    println!("Registered students:");
    println!("Alice ID: {}", alice_id);
    println!("Bob ID: {}", bob_id);
    
    // View a student
    match manager.view_student(alice_id) {
        Ok(student) => println!("Alice: {:?}", student),
        Err(e) => println!("Error: {}", e),
    }
    
    // Update a student
    manager.update_student(
        alice_id,
        Some("Alice Williams".to_string()),
        Some(88.0),
        Some(StudentStatus::Inactive),
    ).unwrap();
    
    println!("After update:");
    println!("Alice: {:?}", manager.view_student(alice_id).unwrap());
    
    // View all students
    println!("\nAll students:");
    for (id, student) in manager.view_all_students() {
        println!("ID {}: {:?}", id, student);
    }
    
    // Filter by status
    println!("\nActive students:");
    for (id, student) in manager.filter_by_status(StudentStatus::Active) {
        println!("ID {}: {}", id, student.name);
    }
}