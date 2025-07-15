pub mod lib;
use crate::lib::{Todo, TodoItem};

fn main() {
    let mut student = Todo::initialize();

    let name = "Ebele Okolo".to_string();
    let grade = 12;
    let is_active = true;

    student.create_student(name, grade, is_active);

    let student = student.get_student();

    println!("Student are {:#?}", student);
}