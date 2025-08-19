use task_1::{ClassEnrollment, Student};

fn main() {
    println!("Hello, world!");
    let mut enrollment = ClassEnrollment::new();
    let student = Student::new(1, "Alice".to_string(), 90);
    enrollment.add_student(student.clone());

    println!("{:?}", enrollment.students.len())
}
