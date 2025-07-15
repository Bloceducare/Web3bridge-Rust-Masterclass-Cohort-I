use class_hub::{ClassManagementSystem, StudentStatus};

fn main() {
    let mut system = ClassManagementSystem::new();

    println!("=== Class Management System Demo ===");

    // Register students
    match system.register_student("John Smith".to_string(), "10th".to_string()) {
        Ok(id) => println!("Registered student with ID: {}", id),
        Err(e) => println!("Error: {}", e),
    }

    match system.register_student("Jane Doe".to_string(), "9th".to_string()) {
        Ok(id) => println!("Registered student with ID: {}", id),
        Err(e) => println!("Error: {}", e),
    }

    // View all students
    println!("\nAll students:");
    for student in system.view_all_students() {
        println!("{}", student);
    }

    // Edit a student
    if let Err(e) = system.edit_student(1, Some("Jonathan Smith".to_string()), None, None) {
        println!("Error editing student: {}", e);
    }

    // Update student status
    if let Err(e) = system.update_student_status(2, StudentStatus::Inactive) {
        println!("Error updating status: {}", e);
    }

    // View updated students
    println!("\nUpdated students:");
    for student in system.view_all_students() {
        println!("{}", student);
    }

    // View students by status
    println!("\nActive students:");
    for student in system.view_students_by_status(StudentStatus::Active) {
        println!("{}", student);
    }

    println!("\nInactive students:");
    for student in system.view_students_by_status(StudentStatus::Inactive) {
        println!("{}", student);
    }

    println!("\nTotal students: {}", system.student_count());
}
