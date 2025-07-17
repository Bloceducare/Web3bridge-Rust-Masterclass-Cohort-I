#[derive(Debug, Clone)]
struct Student {
    id: i32,
    name: String,
    grade: u8,
    status: Status,
}
#[derive(Debug, Clone)]
enum Status {
    Active,
    Inactive,
}
fn register_student( id:i32,students: &mut Vec<Student>, name: String, grade: u8) {
    let student = Student {
        id,
        name,
        grade,
        status: Status::Active,
    };
    students.push(student);
}
fn edit_student(id:i32, students: &mut Vec<Student>, index: usize, new_name: String, new_grade: u8) {
    if let Some(student) = students.get_mut(index) {
        student.name = new_name;
        student.grade = new_grade;
    }
}
fn update_status(id:i32,students: &mut Vec<Student>, index: usize, status: Status) {
    if let Some(student) = students.get_mut(index) {
        student.status = status;
    }
}
fn delete_student(id:i32,students: &mut Vec<Student>, index: usize) {
    if index < students.len() {
        students.remove(index);
    }
}
fn view_students(students: &Vec<Student>) {
    for (i, student) in students.iter().enumerate() {
        println!("{}: {:?}", i, student);
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    
    register_student(&mut students, "Alice".to_string(), 10);
    register_student(&mut students, "Bob".to_string(), 9);

   
    println!("All students:");
    view_students(&students);

   
    edit_student(&mut students, 1, "Bobby".to_string(), 11);

   
    update_status(&mut students, 0, Status::Inactive);

    
    delete_student(&mut students, 1);

    
    println!("\nFinal student list:");
    view_students(&students);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut students = Vec::new();
        register_student(&mut students, "Test Student".to_string(), 8);

        assert_eq!(students.len(), 1);
        assert_eq!(students[0].name, "Test Student");
        assert_eq!(students[0].grade, 8);
        match students[0].status {
            Status::Active => {}
            _ => panic!("Status should be Active"),
        }
    }

    #[test]
    fn test_edit_student() {
        let mut students = vec![
            Student {
                name: "Original".to_string(),
                grade: 6,
                status: Status::Active,
            }
        ];

        edit_student(&mut students, 0, "Edited".to_string(), 9);

        assert_eq!(students[0].name, "Edited");
        assert_eq!(students[0].grade, 9);
    }

    #[test]
    fn test_update_status() {
        let mut students = vec![
            Student {
                name: "Alex".to_string(),
                grade: 10,
                status: Status::Active,
            }
        ];

        update_status(&mut students, 0, Status::Inactive);

        match students[0].status {
            Status::Inactive => {}
            _ => panic!("Status should be Inactive"),
        }
    }

    #[test]
    fn test_delete_student() {
        let mut students = vec![
            Student {
                name: "A".to_string(),
                grade: 5,
                status: Status::Active,
            },
            Student {
                name: "B".to_string(),
                grade: 6,
                status: Status::Active,
            }
        ];

        delete_student(&mut students, 0);

        assert_eq!(students.len(), 1);
        assert_eq!(students[0].name, "B");
    }

    #[test]
    fn test_view_students_output() {
        let students = vec![
            Student {
                name: "Viewer".to_string(),
                grade: 7,
                status: Status::Active,
            }
        ];

       
        view_students(&students);
    }
}