use class_management_system::{Student, StudentManager, StudentStatus};
use std::io::{self, Write};

fn main() {
    println!("=== Class Management System ===");
    println!("Created by Sandra\n");

    let mut manager = StudentManager::new();

    // Demonstrate the system with some sample data
    demonstrate_system(&mut manager);

    // Interactive CLI
    run_interactive_cli(&mut manager);
}

fn demonstrate_system(manager: &mut StudentManager) {
    println!("🎓 Demonstrating Class Management System Features\n");

    // Register some students
    println!("📝 Registering students...");
    let _ = manager.register_student("Alice Johnson".to_string(), 92.5);
    let _ = manager.register_student("Bob Smith".to_string(), 78.0);
    let _ = manager.register_student("Charlie Brown".to_string(), 85.5);
    let _ = manager.register_student("Diana Prince".to_string(), 96.0);

    // Display all students
    println!("\n👥 All Students:");
    print_students(manager.view_all_students());

    // Edit a student
    println!("\n✏️ Editing Bob's grade to 82.0...");
    let _ = manager.edit_student(2, None, Some(82.0));

    // Update student status
    println!("🔄 Making Charlie inactive...");
    let _ = manager.update_student_status(3, StudentStatus::Inactive);

    // Display updated information
    println!("\n👥 Updated Student List:");
    print_students(manager.view_all_students());

    println!("\n✅ Active Students:");
    print_students(manager.view_active_students());

    println!("\n❌ Inactive Students:");
    print_students(manager.view_inactive_students());

    // Show statistics
    print_statistics(manager);

    println!("\n{}", "=".repeat(50));
}

fn run_interactive_cli(manager: &mut StudentManager) {
    loop {
        print_menu();
        
        let choice = read_input("Enter your choice: ");
        
        match choice.trim() {
            "1" => register_student_interactive(manager),
            "2" => edit_student_interactive(manager),
            "3" => update_status_interactive(manager),
            "4" => delete_student_interactive(manager),
            "5" => view_student_interactive(manager),
            "6" => view_all_students_interactive(manager),
            "7" => view_active_students_interactive(manager),
            "8" => view_inactive_students_interactive(manager),
            "9" => print_statistics(manager),
            "0" => {
                println!("👋 Thank you for using the Class Management System!");
                break;
            }
            _ => println!("❌ Invalid choice. Please try again."),
        }
        
        println!(); // Add spacing
    }
}

fn print_menu() {
    println!("\n📋 Class Management System Menu:");
    println!("1. Register Student");
    println!("2. Edit Student");
    println!("3. Update Student Status");
    println!("4. Delete Student");
    println!("5. View Student");
    println!("6. View All Students");
    println!("7. View Active Students");
    println!("8. View Inactive Students");
    println!("9. Show Statistics");
    println!("0. Exit");
    println!("{}", "-".repeat(30));
}

fn register_student_interactive(manager: &mut StudentManager) {
    let name = read_input("Enter student name: ");
    let grade_str = read_input("Enter student grade (0-100): ");
    
    match grade_str.trim().parse::<f64>() {
        Ok(grade) => {
            match manager.register_student(name, grade) {
                Ok(id) => println!("✅ Student registered successfully with ID: {}", id),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        Err(_) => println!("❌ Invalid grade. Please enter a number."),
    }
}

fn edit_student_interactive(manager: &mut StudentManager) {
    let id_str = read_input("Enter student ID: ");
    
    match id_str.trim().parse::<u32>() {
        Ok(id) => {
            let name_input = read_input("Enter new name (or press Enter to skip): ");
            let grade_input = read_input("Enter new grade (or press Enter to skip): ");
            
            let new_name = if name_input.trim().is_empty() { 
                None 
            } else { 
                Some(name_input) 
            };
            
            let new_grade = if grade_input.trim().is_empty() {
                None
            } else {
                match grade_input.trim().parse::<f64>() {
                    Ok(grade) => Some(grade),
                    Err(_) => {
                        println!("❌ Invalid grade format.");
                        return;
                    }
                }
            };
            
            match manager.edit_student(id, new_name, new_grade) {
                Ok(_) => println!("✅ Student updated successfully!"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        Err(_) => println!("❌ Invalid ID. Please enter a number."),
    }
}

fn update_status_interactive(manager: &mut StudentManager) {
    let id_str = read_input("Enter student ID: ");
    
    match id_str.trim().parse::<u32>() {
        Ok(id) => {
            println!("Select status:");
            println!("1. Active");
            println!("2. Inactive");
            
            let status_choice = read_input("Enter choice (1 or 2): ");
            
            let status = match status_choice.trim() {
                "1" => StudentStatus::Active,
                "2" => StudentStatus::Inactive,
                _ => {
                    println!("❌ Invalid choice.");
                    return;
                }
            };
            
            match manager.update_student_status(id, status) {
                Ok(_) => println!("✅ Student status updated successfully!"),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        Err(_) => println!("❌ Invalid ID. Please enter a number."),
    }
}

fn delete_student_interactive(manager: &mut StudentManager) {
    let id_str = read_input("Enter student ID to delete: ");
    
    match id_str.trim().parse::<u32>() {
        Ok(id) => {
            let confirmation = read_input("Are you sure? (y/N): ");
            
            if confirmation.trim().to_lowercase() == "y" {
                match manager.delete_student(id) {
                    Ok(student) => println!("✅ Student deleted: {}", student),
                    Err(e) => println!("❌ Error: {}", e),
                }
            } else {
                println!("🚫 Deletion cancelled.");
            }
        }
        Err(_) => println!("❌ Invalid ID. Please enter a number."),
    }
}

fn view_student_interactive(manager: &StudentManager) {
    let id_str = read_input("Enter student ID: ");
    
    match id_str.trim().parse::<u32>() {
        Ok(id) => {
            match manager.view_student(id) {
                Ok(student) => {
                    println!("📋 Student Details:");
                    println!("  {}", student);
                }
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        Err(_) => println!("❌ Invalid ID. Please enter a number."),
    }
}

fn view_all_students_interactive(manager: &StudentManager) {
    let students = manager.view_all_students();
    if students.is_empty() {
        println!("📭 No students registered.");
    } else {
        println!("👥 All Students ({}):", students.len());
        print_students(students);
    }
}

fn view_active_students_interactive(manager: &StudentManager) {
    let students = manager.view_active_students();
    if students.is_empty() {
        println!("📭 No active students.");
    } else {
        println!("✅ Active Students ({}):", students.len());
        print_students(students);
    }
}

fn view_inactive_students_interactive(manager: &StudentManager) {
    let students = manager.view_inactive_students();
    if students.is_empty() {
        println!("📭 No inactive students.");
    } else {
        println!("❌ Inactive Students ({}):", students.len());
        print_students(students);
    }
}

fn print_students(students: Vec<&Student>) {
    for student in students {
        println!("  {}", student);
    }
}

fn print_statistics(manager: &StudentManager) {
    println!("\n📊 Class Statistics:");
    println!("  Total Students: {}", manager.total_students());
    println!("  Active Students: {}", manager.active_students_count());
    println!("  Inactive Students: {}", manager.inactive_students_count());
    
    if let Some(avg) = manager.average_grade() {
        println!("  Overall Average Grade: {:.2}", avg);
    }
    
    if let Some(avg) = manager.average_grade_active() {
        println!("  Active Students Average Grade: {:.2}", avg);
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
