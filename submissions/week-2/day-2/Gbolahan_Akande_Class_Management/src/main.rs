pub mod lib;
use crate::lib::{Management, Student, StudentStatus};

fn main() {
    let mut stud = Management::initialize();

    let name_one = "Alice".to_string();
    let grade_one = 85;

    let name_two = "Bob".to_string();
    let grade_two = 92;

    stud.register_st(name_one, grade_one);
    stud.register_st(name_two, grade_two);

    let students = stud.get_students();

    println!("Students are {:#?}", students);
}
