extern crate school_management_system;

use school_management_system::{School, StudentStatus};

#[test]
fn test_new_school() {
    let school = School::new();
    assert_eq!(school.students.len(), 0);
}

#[test]
fn test_register_student() {
    let mut school = School::new();
    school.register_student(String::from("John"), 10, String::from("male"));
    assert_eq!(school.students.len(), 1);
    assert_eq!(school.students[0].name, "John");
    assert_eq!(school.students[0].grade, 10);
    assert_eq!(school.students[0].gender, "male");
    assert_eq!(school.students[0].status, StudentStatus::ACTIVE);
}

#[test]
fn test_get_student() {
    let mut school = School::new();
    school.register_student(String::from("Jane"), 11, String::from("female"));
    let student = school.get_student(0);
    assert_eq!(student.name, "Jane");
    assert_eq!(student.grade, 11);
    assert_eq!(student.gender, "female");
    assert_eq!(student.status, StudentStatus::ACTIVE);
}

#[test]
fn test_get_student_by_name() {
    let mut school = School::new();
    school.register_student(String::from("Jane"), 11, String::from("female"));
    let student = school.get_student_by_name(String::from("Jane"));
    assert!(student.is_some());
    let student = student.unwrap();
    assert_eq!(student.name, "Jane");
}

#[test]
fn test_get_all_students() {
    let mut school = School::new();
    school.register_student(String::from("John"), 10, String::from("male"));
    school.register_student(String::from("Jane"), 11, String::from("female"));
    let students = school.get_all_students();
    assert_eq!(students.len(), 2);
}

#[test]
fn test_update_student() {
    let mut school = School::new();
    school.register_student(String::from("John"), 10, String::from("male"));
    school.update_student(0, String::from("Johnny"), 12, String::from("male"));
    let student = school.get_student(0);
    assert_eq!(student.name, "Johnny");
    assert_eq!(student.grade, 12);
    assert_eq!(student.gender, "male");
    assert_eq!(student.status, StudentStatus::ACTIVE);
}

#[test]
fn test_delete_student() {
    let mut school = School::new();
    school.register_student(String::from("John"), 10, String::from("male"));
    school.register_student(String::from("Jane"), 11, String::from("female"));
    school.delete_student(0);
    assert_eq!(school.students.len(), 1);
    assert_eq!(school.students[0].name, "Jane");
}
