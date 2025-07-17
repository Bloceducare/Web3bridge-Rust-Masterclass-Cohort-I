pub mod lib;
use crate::lib::{SchoolDB,Student, Status};

fn main() {
    println!("Welcome to Okoro Schools");
    let mut school_register = SchoolDB::new();

    assert!(school_register.register.len() == 0);

    let student = Student {
      name: "Glory Praise".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    let student2 = Student {
      name: "Paul Peter".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::False,
    };

    school_register.add_new_student(student);
    school_register.add_new_student(student2);

    let student = school_register.view_student(0);
    println!("\nFirst student registered is {:#?}", student);

    let students = school_register.view_students();
    println!("\nTotal student registered are {:#?}", students);

     let new_student = Student {
      name: "John Rust".to_string(),
      grade: "Grade B".to_string(),
      is_active: Status::True,
    };

    school_register.update_student(0, new_student);
    
    let student = school_register.view_student(0);
    println!("\nFirst student data updated {:#?}", student);

    school_register.delete_student(1);

    let students = school_register.view_students();
    println!("\nTotal student registered are {:#?}", students);


}
