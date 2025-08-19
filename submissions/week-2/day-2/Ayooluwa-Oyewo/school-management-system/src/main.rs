/*
A class management system that has the name of the student, grade, enum that tracks if student is active or not.

Have the following functions:
•⁠  ⁠Function to register student
•⁠  ⁠Edit
•⁠  ⁠Update
•⁠  ⁠Delete functions
•⁠  ⁠View function
*/

#[derive(Debug, PartialEq)]
enum StudentStatus {
    Active,
    Inactive,
}

#[derive(Debug, PartialEq)]
enum Grade {
    A,
    B,
    C,
    D,
    F,
}
#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    grade: Grade,
    status: StudentStatus,
}

struct SchoolManagementSystem {
    students: Vec<Student>,
}

impl SchoolManagementSystem {
    fn new() -> Self {
        Self {
            students: Vec::new(),
        }
    }
    fn register_student(&mut self, student: Student) {
        self.students.push(student);
    }
    fn update_student(&mut self, updated_student: Student) {
        if let Some(student) = self
            .students
            .iter_mut()
            .find(|s| s.id == updated_student.id)
        {
            student.name = updated_student.name;
            student.grade = updated_student.grade;
            student.status = updated_student.status;
        } else {
            println!("Student with id {} not found", updated_student.id);
        }
    }
    fn delete_student(&mut self, id: u32) {
        self.students.retain(|s| s.id != id);
    }
    fn view_student(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == id)
    }
    fn update_student_status_by_id(&mut self, id: u32, status: StudentStatus) -> Option<&Student> {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.status = status;
            println!("Updated student {}'s status to {:?}", student.name, student.status);
            Some(student)
        } else {
            println!("No student found with ID {}", id);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_register_student() {
        let mut school = SchoolManagementSystem::new();
        let student = Student {
            id: 1,
            name: "John Doe".to_string(),
            grade: Grade::A,
            status: StudentStatus::Active,
        };
        school.register_student(student);
        assert_eq!(school.students.len(), 1);
        assert_eq!(school.students[0].name, "John Doe");
        assert_eq!(school.students[0].grade, Grade::A);
        assert_eq!(school.students[0].status, StudentStatus::Active);
    }
    #[test]
    fn test_update_student() {
        let mut school = SchoolManagementSystem::new();
        let student = Student {
            id: 1,
            name: "John Doe".to_string(),
            grade: Grade::A,
            status: StudentStatus::Active,
        };
        school.register_student(student);
        let updated_student = Student {
            id: 1,
            name: "Jane Doe".to_string(),
            grade: Grade::B,
            status: StudentStatus::Inactive,
        };
        school.update_student(updated_student);
        assert_eq!(school.students.len(), 1);
        assert_eq!(school.students[0].name, "Jane Doe");
        assert_eq!(school.students[0].grade, Grade::B);
        assert_eq!(school.students[0].status, StudentStatus::Inactive);
    }
    #[test]
    fn test_view_student() {
        let mut school = SchoolManagementSystem::new();
        let student = Student {
            id: 1,
            name: "John Doe".to_string(),
            grade: Grade::A,
            status: StudentStatus::Active,
        };
        school.register_student(student);
        assert_eq!(school.view_student(1).unwrap().name, "John Doe");
    }
    #[test]
    fn test_delete_student() {
        let mut school = SchoolManagementSystem::new();
        let student = Student {
            id: 1,
            name: "John Doe".to_string(),
            grade: Grade::A,
            status: StudentStatus::Active,
        };
        school.register_student(student);
        school.delete_student(1);
        assert_eq!(school.students.len(), 0);
    }
    #[test]
    fn test_update_student_status_by_id() {
        let mut school = SchoolManagementSystem::new();
        let student = Student {
            id: 1,
            name: "John Doe".to_string(),
            grade: Grade::A,
            status: StudentStatus::Active,
        };
        school.register_student(student);
        school.update_student_status_by_id(1, StudentStatus::Active);
        assert_eq!(school.students.len(), 1);
        assert_eq!(school.students[0].name, "John Doe");
        assert_eq!(school.students[0].grade, Grade::A);
        assert_eq!(school.students[0].status, StudentStatus::Active);
    }
}
fn main() {
    let mut school = SchoolManagementSystem::new();
    let student = Student {
        id: 1,
        name: "John Doe".to_string(),
        grade: Grade::A,
        status: StudentStatus::Active,
    };
    school.register_student(student);
    println!("Registered student: {:?}", school.students);
    let updated_student = Student {
        id: 1,
        name: "John Doe".to_string(),
        grade: Grade::B,
        status: StudentStatus::Inactive,
    };
    school.update_student(updated_student);
    println!("Updated student: {:?}", school.students);
    school.view_student(1);
    println!("Viewed student: {:?}", school.students);
    school.update_student_status_by_id(1, StudentStatus::Active);

    school.delete_student(1);
    println!("Deleted student: {:?}", school.students);
}
