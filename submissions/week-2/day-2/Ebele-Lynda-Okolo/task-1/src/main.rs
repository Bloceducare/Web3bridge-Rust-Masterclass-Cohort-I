use task_1::Student;

fn main() {
    let mut student = Student::new("Ebele Okolo".to_string(), 12, true);

    student.edit("Lynda Okolo".to_string(), 12, true);

    student.view();
}