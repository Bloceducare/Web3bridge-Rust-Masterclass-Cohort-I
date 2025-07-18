use crate::management_system::ClassManagementSystem;
use crate::student::StudentStatus;
use std::io::{self, Write};

pub struct UserInterface {
    cms: ClassManagementSystem,
}

impl UserInterface {
    pub fn new() -> Self {
        UserInterface {
            cms: ClassManagementSystem::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.display_menu();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            match input.trim() {
                "1" => self.handle_register_student(),
                "2" => self.handle_edit_student(),
                "3" => self.handle_update_student(),
                "4" => self.handle_delete_student(),
                "5" => self.handle_view_student(),
                "6" => self.handle_view_all_students(),
                "7" => {
                    println!("Goodbye!");
                    break;
                }
                _ => println!("Invalid option! Please choose 1-7."),
            }
        }
    }

    fn display_menu(&self) {
        println!("\n=== Class Management System ===");
        println!("1. Register Student");
        println!("2. Edit Student");
        println!("3. Update Student");
        println!("4. Delete Student");
        println!("5. View Student");
        println!("6. View All Students");
        println!("7. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();
    }

    fn read_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    fn handle_register_student(&mut self) {
        let name = Self::read_input("Enter student name: ");
        let grade = Self::read_input("Enter student grade: ");

        match self.cms.register_student(name, grade) {
            Ok(id) => println!("Student registered with ID: {}", id),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn handle_edit_student(&mut self) {
        let id_input = Self::read_input("Enter student ID to edit: ");
        
        if let Ok(id) = id_input.parse::<u32>() {
            let name_input = Self::read_input("Enter new name (press Enter to skip): ");
            let name = if name_input.is_empty() { None } else { Some(name_input) };

            let grade_input = Self::read_input("Enter new grade (press Enter to skip): ");
            let grade = if grade_input.is_empty() { None } else { Some(grade_input) };

            let status_input = Self::read_input("Enter new status (active/inactive, press Enter to skip): ");
            let status = StudentStatus::from_string(&status_input);

            match self.cms.edit_student(id, name, grade, status) {
                Ok(()) => println!("Student updated successfully!"),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Invalid ID format!");
        }
    }

    fn handle_update_student(&mut self) {
        let id_input = Self::read_input("Enter student ID to update: ");
        
        if let Ok(id) = id_input.parse::<u32>() {
            let name = Self::read_input("Enter new name: ");
            let grade = Self::read_input("Enter new grade: ");
            let status_input = Self::read_input("Enter status (active/inactive): ");
            
            let status = StudentStatus::from_string(&status_input).unwrap_or_else(|| {
                println!("Invalid status! Using Active as default.");
                StudentStatus::Active
            });

            match self.cms.update_student(id, name, grade, status) {
                Ok(()) => println!("Student updated successfully!"),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Invalid ID format!");
        }
    }

    fn handle_delete_student(&mut self) {
        let id_input = Self::read_input("Enter student ID to delete: ");
        
        if let Ok(id) = id_input.parse::<u32>() {
            match self.cms.delete_student(id) {
                Ok(student) => println!("Student '{}' deleted successfully!", student.name),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Invalid ID format!");
        }
    }

    fn handle_view_student(&self) {
        let id_input = Self::read_input("Enter student ID to view: ");
        
        if let Ok(id) = id_input.parse::<u32>() {
            if let Some(student) = self.cms.view_student(id) {
                println!("Student Details:");
                println!("ID: {}", student.id);
                println!("Name: {}", student.name);
                println!("Grade: {}", student.grade);
                println!("Status: {:?}", student.status);
            } else {
                println!("Student not found!");
            }
        } else {
            println!("Invalid ID format!");
        }
    }

    fn handle_view_all_students(&self) {
        let students = self.cms.view_all_students();
        if students.is_empty() {
            println!("No students registered.");
        } else {
            println!("All Students:");
            for student in students {
                println!("ID: {}, Name: {}, Grade: {}, Status: {:?}", 
                        student.id, student.name, student.grade, student.status);
            }
        }
    }
}

impl Default for UserInterface {
    fn default() -> Self {
        Self::new()
    }
}