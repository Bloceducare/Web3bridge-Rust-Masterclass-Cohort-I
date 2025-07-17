use task_1::{Student, Status};

fn main() {
    let mut student = Student::new("Ebele Okolo".to_string(), 12, Status::Active);

    student.edit("Lynda Okolo".to_string(), 12, Status::Active);

    student.view();
}