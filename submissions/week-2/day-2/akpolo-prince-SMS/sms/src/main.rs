// A class management system that has the name of the student, grade, enum that tracks if student is active or not.

// Have the following functions:
// - Function to register student
// - Edit
// - Update
// - Delete functions
// - View function

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Status {
    Active,
    Inactive,
}

#[derive(Debug)]
pub struct Student {
    name: String,
    grade: u8,
    status: Status,
}

pub struct ClassManagement {
    students: Vec<Student>,
}

impl ClassManagement {
    fn new() -> Self {
        ClassManagement {
            students: Vec::new(),
        }
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn remove_student(&mut self, name: &str) {
        self.students.retain(|s| s.name != name);
    }

    fn view_students(&self) {
        for student in &self.students {
            println!("{:?}", student);
        }
    }

    fn edit_student(
        &mut self,
        name: &str,
        new_name: Option<&str>,
        new_grade: Option<u8>,
        new_status: Option<Status>,
    ) {
        for student in &mut self.students {
            if student.name == name {
                if let Some(n) = new_name {
                    student.name = n.to_string();
                }
                if let Some(g) = new_grade {
                    student.grade = g;
                }
                if let Some(s) = new_status {
                    student.status = s;
                }
            }
        }
    }

    fn update_student(
        &mut self,
        name: &str,
        new_name: Option<&str>,
        new_grade: Option<u8>,
        new_status: Option<Status>,
    ) {
        self.edit_student(name, new_name, new_grade, new_status);
    }
}

fn main() {
    let mut class_management = ClassManagement::new();

    class_management.add_student(Student {
        name: String::from("John Doe"),
        grade: 11,
        status: Status::Active,
    });

    let student1 = Student {
        name: String::from("Alice"),
        grade: 10,
        status: Status::Active,
    };

    let student2 = Student {
        name: String::from("Bob"),
        grade: 12,
        status: Status::Inactive,
    };

    class_management.add_student(student1);
    class_management.add_student(student2);
    class_management.edit_student(
        "Alice",
        Some("Alice Smith"),
        Some(11),
        Some(Status::Inactive),
    );

    class_management.view_students();

    class_management.remove_student("John Doe");
    class_management.update_student(
        "Lynda joseph",
        Some("Lynda pope"),
        Some(12),
        Some(Status::Active),
    );
    class_management.view_students();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_student_management() {
        let mut class_management = ClassManagement::new();

        class_management.add_student(Student {
            name: String::from("Josh tutor"),
            grade: 11,
            status: Status::Active,
        });

        class_management.add_student(Student {
            name: String::from("prince akpolo"),
            grade: 10,
            status: Status::Active,
        });

        class_management.add_student(Student {
            name: String::from("Bobrisky"),
            grade: 12,
            status: Status::Inactive,
        });
        debug_assert!(class_management.students.len() == 3);
    }

    #[test]
    fn test_remove_student_management() {
        let mut class_management = ClassManagement::new();

        class_management.add_student(Student {
            name: String::from("Josh tutor"),
            grade: 11,
            status: Status::Active,
        });

        class_management.add_student(Student {
            name: String::from("prince akpolo"),
            grade: 10,
            status: Status::Active,
        });

        class_management.add_student(Student {
            name: String::from("Bobrisky"),
            grade: 12,
            status: Status::Inactive,
        });

        class_management.remove_student("prince akpolo");
        class_management.view_students();
        debug_assert!(class_management.students.len() == 2);
    }
    #[test]
    fn test_edit_student_management() {
        let mut class_management = ClassManagement::new();

        class_management.add_student(Student {
            name: String::from("Josh tutor"),
            grade: 11,
            status: Status::Active,
        });

        class_management.edit_student(
            "Josh tutor",
            Some("Joshua Tutor"),
            Some(12),
            Some(Status::Inactive),
        );

        let student = class_management.students.iter().find(|s| s.name == "Joshua Tutor");
        assert!(student.is_some());
        assert_eq!(student.unwrap().grade, 12);
        assert_eq!(student.unwrap().status, Status::Inactive);
    }
    #[test]
    fn test_update_student_management() {
        let mut class_management = ClassManagement::new();

        class_management.add_student(Student {
            name: String::from("Josh tutor"),
            grade: 11,
            status: Status::Active,
        });

        class_management.update_student(
            "Josh tutor",
            Some("Joshua Tutor"),
            Some(12),
            Some(Status::Inactive),
        );

        let student = class_management.students.iter().find(|s| s.name == "Joshua Tutor");
        assert!(student.is_some());
        assert_eq!(student.unwrap().grade, 12);
        assert_eq!(student.unwrap().status, Status::Inactive);
    }
    #[test]
    fn test_view_students_management() {
        let mut class_management = ClassManagement::new();

        class_management.add_student(Student {
            name: String::from("Josh tutor"),
            grade: 11,
            status: Status::Active,
        });

        class_management.add_student(Student {
            name: String::from("prince akpolo"),
            grade: 10,
            status: Status::Active,
        });

        class_management.add_student(Student {
            name: String::from("Bobrisky"),
            grade: 12,
            status: Status::Inactive,
        });

        class_management.view_students();
        debug_assert!(class_management.students.len() == 3);
    }


}

