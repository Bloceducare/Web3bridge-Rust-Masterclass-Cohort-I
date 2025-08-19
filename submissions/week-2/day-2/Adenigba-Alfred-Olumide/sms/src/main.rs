pub mod lib;
use crate::lib::{School, Status};

fn main() {
    let mut school = School::new();

    school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);
    school.create_student("Olaide Adenigba".to_string(), 12, "SS1".to_string(), Status::Active);
    school.create_student("Olamide Adenigba".to_string(), 9, "JSS1".to_string(), Status::Active);
    school.create_student("Ayomide Adenigba".to_string(), 6, "PRY4".to_string(), Status::Active);

    let students = school.get_students();

    println!("Students are {:#?}", students);
}