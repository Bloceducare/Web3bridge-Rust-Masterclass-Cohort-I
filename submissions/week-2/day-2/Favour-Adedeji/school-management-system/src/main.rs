pub mod lib;
use crate::lib::{School, Student, Active};

fn main() {
    let mut school = School::initialize();

    let student = Student {
        name: "Rick".to_string(),
        grade: "class 5".to_string(),
        active: Active::TRUE
    };

    let student2 = Student {
        name: "John".to_string(),
        grade: "class 6".to_string(),
        active: Active::FALSE
    };

    school.create_student(student);
    school.create_student(student2);

    let students = school.get_students();

    println!("Students are {:#?}", students);
}
