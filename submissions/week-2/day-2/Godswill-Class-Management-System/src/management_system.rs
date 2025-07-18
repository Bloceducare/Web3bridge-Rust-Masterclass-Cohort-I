use crate::student::{Student, StudentStatus};
use std::collections::HashMap;

pub struct ClassManagementSystem {
    students: HashMap<u32, Student>,
    next_id: u32,
}

impl ClassManagementSystem {
    pub fn new() -> Self {
        ClassManagementSystem {
            students: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn register_student(&mut self, name: String, grade: String) -> Result<u32, String> {
        if name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if grade.trim().is_empty() {
            return Err("Grade cannot be empty".to_string());
        }

        let student = Student::new(self.next_id, name, grade);
        self.students.insert(self.next_id, student);
        let id = self.next_id;
        self.next_id += 1;
        Ok(id)
    }

    pub fn edit_student(
        &mut self, 
        id: u32, 
        name: Option<String>, 
        grade: Option<String>, 
        status: Option<StudentStatus>
    ) -> Result<(), String> {
        match self.students.get_mut(&id) {
            Some(student) => {
                if let Some(ref n) = name {
                    if n.trim().is_empty() {
                        return Err("Name cannot be empty".to_string());
                    }
                }
                if let Some(ref g) = grade {
                    if g.trim().is_empty() {
                        return Err("Grade cannot be empty".to_string());
                    }
                }
                student.edit(name, grade, status);
                Ok(())
            }
            None => Err("Student not found".to_string()),
        }
    }

    pub fn update_student(
        &mut self, 
        id: u32, 
        name: String, 
        grade: String, 
        status: StudentStatus
    ) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if grade.trim().is_empty() {
            return Err("Grade cannot be empty".to_string());
        }

        match self.students.get_mut(&id) {
            Some(student) => {
                student.update(name, grade, status);
                Ok(())
            }
            None => Err("Student not found".to_string()),
        }
    }

    pub fn delete_student(&mut self, id: u32) -> Result<Student, String> {
        self.students.remove(&id).ok_or("Student not found".to_string())
    }

    pub fn view_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    pub fn view_all_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }

    pub fn get_students_by_status(&self, status: &StudentStatus) -> Vec<&Student> {
        self.students
            .values()
            .filter(|student| &student.status == status)
            .collect()
    }

    pub fn get_students_by_grade(&self, grade: &str) -> Vec<&Student> {
        self.students
            .values()
            .filter(|student| student.grade == grade)
            .collect()
    }

    pub fn student_count(&self) -> usize {
        self.students.len()
    }

    pub fn update_student_status(&mut self, id: u32, status: StudentStatus) -> Result<(), String> {
        match self.students.get_mut(&id) {
            Some(student) => {
                student.status = status;
                Ok(())
            }
            None => Err("Student not found".to_string()),
        }
    }

    pub fn toggle_student_status(&mut self, id: u32) -> Result<(), String> {
        match self.students.get_mut(&id) {
            Some(student) => {
                student.status = match student.status {
                    StudentStatus::Active => StudentStatus::Inactive,
                    StudentStatus::Inactive => StudentStatus::Active,
                };
                Ok(())
            }
            None => Err("Student not found".to_string()),
        }
    }
}

impl Default for ClassManagementSystem {
    fn default() -> Self {
        Self::new()
    }
}