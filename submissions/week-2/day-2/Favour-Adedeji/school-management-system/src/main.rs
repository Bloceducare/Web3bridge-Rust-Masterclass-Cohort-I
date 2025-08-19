pub mod lib;
use crate::lib::{School, Student, Active};

fn main() {
    let mut school = School::initialize();

    let student = Student {
        id: 1,
        name: "Rick".to_string(),
        grade: "class 5".to_string(),
        status: Active::ACTIVE
    };

    let student2 = Student {
        id: 2,
        name: "John".to_string(),
        grade: "class 6".to_string(),
        status: Active::INACTIVE
    };

    school.create_student(student);
    school.create_student(student2);
    
    school.update_status(2);

    let students = school.get_students();


    println!("Students are {:#?}", students);
}
