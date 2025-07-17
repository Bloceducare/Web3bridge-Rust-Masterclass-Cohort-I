use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grade: u32,
    pub status: Status
}

impl Student {
    pub fn new(id: u32, name: String, grade: u32) -> Self {
        Student {
            id, 
            name,
            grade,
            status: Status::Active,
        }
    }
}

pub struct ClassEnrollment {
    pub students: HashMap<u32, Student>,
}

impl ClassEnrollment {
    pub fn new() -> Self {
        ClassEnrollment {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.insert(student.id, student);
    }

    pub fn remove_student(&mut self, id: u32) -> Option<Student> {
        self.students.remove(&id)
    }

    pub fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    pub fn update_student_status(&mut self, id: u32, status: Status) -> Option<&Student> {
        if let Some(student) = self.students.get_mut(&id) {
            student.status = status;
            Some(student)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_student_creation() {
        let student = Student::new(1, "Alice".to_string(), 90);
        assert_eq!(student.id, 1);
        assert_eq!(student.name, "Alice");
        assert_eq!(student.grade, 90);
        assert_eq!(student.status, Status::Active);
    }       

    #[test]
    fn test_class_enrollment() {
        let mut enrollment = ClassEnrollment::new();
        let student = Student::new(1, "Alice".to_string(), 90);
        enrollment.add_student(student.clone());
        assert_eq!(enrollment.get_student(1), Some(&student));      
        assert_eq!(enrollment.students.len(), 1);
        assert_eq!(enrollment.remove_student(1), Some(student));
        assert_eq!(enrollment.get_student(1), None);
    }   

    #[test]
    fn test_update_student_status() {       
        let mut enrollment = ClassEnrollment::new();
        let student = Student::new(1, "Alice".to_string(), 90);
        enrollment.add_student(student.clone());
        assert_eq!(enrollment.update_student_status(1, Status::Active), Some(&student));
        enrollment.update_student_status(1, Status::Inactive);
        assert_eq!(enrollment.get_student(1).unwrap().status, Status::Inactive);
    }

    #[test]
    fn test_remove_nonexistent_student() {
        let mut enrollment = ClassEnrollment::new();
        assert_eq!(enrollment.remove_student(1), None);
    }   
    #[test]
    fn test_get_nonexistent_student() {
        let enrollment = ClassEnrollment::new();
        assert_eq!(enrollment.get_student(1), None);
    }
    #[test]
    fn test_update_status_of_nonexistent_student() {
        let mut enrollment = ClassEnrollment::new();
        assert_eq!(enrollment.update_student_status(1, Status::Inactive), None);
    }
}