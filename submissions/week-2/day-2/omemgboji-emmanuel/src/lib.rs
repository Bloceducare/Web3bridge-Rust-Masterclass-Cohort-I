#[derive(Debug, PartialEq)]
pub enum Status {
    Inactive,
    Active,
}
pub struct Student {
    name: String,
    grade: u8,
    status: Status,
}

pub struct AllStudents {
    students: Vec<Student>,
}

// implementation of methods for AllStudents
impl AllStudents {
    pub fn new() -> Self {
        Self {
            students: Vec::new(),
        }
    }

    pub fn register_student(&mut self, name: String, grade: u8, status: Status) {
        let student = Student {
            name,
            grade,
            status,
        };
        self.students.push(student);
    }

    pub fn get_students(&self) -> &Vec<Student> {
        &self.students
    }

    pub fn get_student(&self, index: usize) -> &Student {
        &self.students.get(index).unwrap()
    }

    pub fn delete_student(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        }
    }

    pub fn update_student(&mut self, index: usize, name: String, grade: u8, status: Status) {
        if let Some(student) = self.students.get_mut(index) {
            student.name = name;
            student.grade = grade;
            student.status = status;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut all_students = AllStudents::new();
        all_students.register_student("Emmanuel".to_string(), 10, Status::Active);
        all_students.register_student("Signor".to_string(), 20, Status::Active);

        assert_eq!(all_students.get_students().len(), 2);
    }

    #[test]
    fn test_get_student() {
        let mut all_students = AllStudents::new();
        all_students.register_student("Emmanuel".to_string(), 10, Status::Active);
        all_students.register_student("Signor".to_string(), 20, Status::Active);

        let student = all_students.get_student(0);
        assert_eq!(student.name, "Emmanuel");
        assert_eq!(student.grade, 10);
        assert_eq!(student.status, Status::Active);
    }

    #[test]
    fn test_delete_student() {
        let mut all_students = AllStudents::new();
        all_students.register_student("Emmanuel".to_string(), 10, Status::Active);
        all_students.register_student("Signor".to_string(), 20, Status::Active);

        all_students.delete_student(0);
        assert_eq!(all_students.get_students().len(), 1);
        assert_eq!(all_students.get_student(0).name, "Signor");
    }

    #[test]
    fn test_update_student() {
        let mut all_students = AllStudents::new();
        all_students.register_student("Emmanuel".to_string(), 10, Status::Active);
        all_students.register_student("Signor".to_string(), 20, Status::Active);

        all_students.update_student(0, "Emmanuel Updated".to_string(), 11, Status::Inactive);
        let student = all_students.get_student(0);
        assert_eq!(student.name, "Emmanuel Updated");
        assert_eq!(student.grade, 11);
        assert_eq!(student.status, Status::Inactive);
    }
}
