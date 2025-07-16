use crate::student::{Student, StudentStatus};
use std::collections::HashMap;

pub struct School {
    pub students: HashMap<u128, Student>,
    pub student_key: u128,
}

impl School {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
            student_key: 0,
        }
    }

    pub fn add_student(&mut self, name: String, grade: u8) -> bool {
        let key = self.student_key + 1;
        let std = Student::new(name, grade);
        self.students.insert(key, std);
        self.student_key += 1;
        true
    }

    pub fn get_student(&self, key: u128) -> Option<&Student> {
        self.students.get(&key)
    }

    pub fn remove_student(&mut self, key: u128) -> Option<Student> {
        self.students.remove(&key)
    }

    pub fn update_student_grade(&mut self, key: u128, grade: u8) -> bool {
        // let mut student = Self::get_student(&self, key).unwrap();
        let student = self.students.get_mut(&key).unwrap();
        student.update_student_grade(grade)
    }
    pub fn update_student_status(&mut self, key: u128, status: StudentStatus) -> bool {
        let std = self.students.get_mut(&key).unwrap();
        std.update_student_status(status)
    }
}
