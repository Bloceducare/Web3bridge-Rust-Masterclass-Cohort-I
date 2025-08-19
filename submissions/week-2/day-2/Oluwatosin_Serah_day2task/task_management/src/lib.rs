#[derive(Clone, Debug, PartialEq)]
pub enum StudentStatus {
    Active,
    Inactive,
}

#[derive(Clone, Debug)]
pub struct Student {
    pub name: String,
    pub grade: u32,
    pub status: StudentStatus,
}

pub struct ClassManagement {
    pub students: Vec<Student>,
}

impl ClassManagement {
    pub fn initialize() -> ClassManagement {
        ClassManagement {
            students: Vec::new(),
        }
    }

    pub fn register_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn register_std(&mut self, name: String, grade: u32) {
        let student = Student {
            name,
            grade,
            status: StudentStatus::Active,
        };

        self.students.push(student);
    }

    pub fn update_student_status(&mut self, index: usize, status: StudentStatus) -> bool {
        if self.students.len() > index {
            self.students[index].status = status;
            true
        } else {
            false
        }
    }

    pub fn delete_student(&mut self, index: usize) -> bool {
        if self.students.len() > index {
            self.students.remove(index);
            true
        } else {
            false
        }
    }

    pub fn view_students(&self) -> Vec<Student> {
        self.students.to_vec()
    }

    pub fn view_student(&self, index: usize) -> &Student {
        self.students.get(index).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut cms = ClassManagement::initialize();
        assert!(cms.students.len() == 0);

        let student = Student {
            name: "Ajala Victor".to_string(),
            grade: 10,
            status: StudentStatus::Active,
        };

        cms.register_student(student);
        assert!(cms.students.len() == 1);
    }

    #[test]
    fn test_register_std() {
        let mut cms = ClassManagement::initialize();
        assert!(cms.students.len() == 0);

        let name = " Serah Oluwatosin".to_string();
        let grade = 95;

        cms.register_std(name, grade);
        assert!(cms.students.len() == 1);
    }


    #[test]
    fn test_view_student() {
        let mut cms = ClassManagement::initialize();
        assert!(cms.students.len() == 0);

        let name = "Blessing".to_string();
        let grade = 100;

        cms.register_std(name, grade);

        let student = cms.view_student(0);

        assert_eq!(student.name, "Blessing".to_string());
        assert_eq!(student.grade, 100);
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_update_student_status() {
        let mut cms = ClassManagement::initialize();
        cms.register_std("Johnson".to_string(), 67);

        let success = cms.update_student_status(0, StudentStatus::Inactive);
        assert!(success);

        let student = cms.view_student(0);
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_delete_student() {
        let mut cms = ClassManagement::initialize();
        cms.register_std("Janet".to_string(), 89);
        cms.register_std("Johnson".to_string(), 40);

        assert_eq!(cms.students.len(), 2);

        let success = cms.delete_student(0);
        assert!(success);
        assert_eq!(cms.students.len(), 1);

        let remaining_student = cms.view_student(0);
        assert_eq!(remaining_student.name, "Johnson");
    }

    #[test]
    fn test_delete_student_out_of_bounds() {
        let mut cms = ClassManagement::initialize();
        cms.register_std("Alice Smith".to_string(), 50);

        let success = cms.delete_student(5);
        assert!(!success);
        assert_eq!(cms.students.len(), 1);
    }
}
