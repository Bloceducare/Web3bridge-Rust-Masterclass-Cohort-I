use school_management_system::{School}; // adjust as needed

fn main() {
    let mut school = School::new();
    school.register_student(String::from("John"), 10, String::from("male"));
    school.register_student(String::from("Jane"), 11, String::from("female"));
    school.register_student(String::from("Jim"), 12, String::from("male"));


    println!("All students: {:?}", school.get_all_students());
    println!("Student by name: {:?}", school.get_student_by_name(String::from("John")));
    println!("Student by index: {:?}", school.get_student(0));
    println!("Student by index: {:?}", school.get_student(1));
    println!("Student by index: {:?}", school.get_student(2));

    school.update_student(0, String::from("John"), 5, String::from("male"));

    // get all students
    let all_students = school.get_all_students();
    println!("All students: {:?}", all_students);

    // get student by name
    let student_by_name = school.get_student_by_name(String::from("John"));
    println!("Student by name: {:?}", student_by_name);

    // get student by index
    let student_by_index = school.get_student(0);
    println!("Student by index: {:?}", student_by_index);



    school.register_student(String::from("Akin"), 12, String::from("male"));

    
    // delete student by index
    school.delete_student(0);
    println!("Student by index: {:?}", school.get_student(0));


    let all_students = school.get_all_students();
    println!("All students: {:?}", all_students);

    // get student by index
    let student_by_index = school.get_student(0);
    println!("Student by index: {:?}", student_by_index);

    // get student by name
    let student_by_name = school.get_student_by_name(String::from("Akin"));
    println!("Student by name: {:?}", student_by_name);
}