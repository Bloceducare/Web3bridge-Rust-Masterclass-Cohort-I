pub mod lib;
use lib::*;

fn main() {
    let mut cms = ClassManagement::initialize();

    cms.register_std("Tammy".to_string(), 89);
    cms.register_std("Jane".to_string(), 23);
    cms.register_std("Mike".to_string(), 50);

    println!("Registered 3 students");

    println!("\nAll students:");
    for (index, student) in cms.view_students().iter().enumerate() {
        println!(
            "  {}: {} - {} (Status: {:?})",
            index, student.name, student.grade, student.status
        );
    }

    let updated_student = cms.view_student(0);

    println!("\nMaking student at index 1 inactive");
    cms.update_student_status(1, StudentStatus::Inactive);


    println!("\nDeleting student at index 2");
    cms.delete_student(2);
}
