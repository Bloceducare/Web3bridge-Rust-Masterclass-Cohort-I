use crate::student::{Student, StudentStatus};
use std::collections::HashMap;

/// Student management system with CRUD operations
#[derive(Debug)]
pub struct StudentManager {
    students: HashMap<u32, Student>,
    next_id: u32,
}

impl StudentManager {
    /// Create a new student manager
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
            next_id: 1,
        }
    }

    /// Register a new student
    pub fn register_student(&mut self, name: String, grade: f64) -> Result<u32, String> {
        if name.trim().is_empty() {
            return Err("Student name cannot be empty".to_string());
        }

        if !(0.0..=100.0).contains(&grade) {
            return Err("Grade must be between 0.0 and 100.0".to_string());
        }

        let id = self.next_id;
        let student = Student::new(id, name, grade);
        self.students.insert(id, student);
        self.next_id += 1;

        Ok(id)
    }

    /// Edit student information (name and/or grade)
    pub fn edit_student(&mut self, id: u32, new_name: Option<String>, new_grade: Option<f64>) -> Result<(), String> {
        let student = self.students.get_mut(&id)
            .ok_or_else(|| format!("Student with ID {} not found", id))?;

        if let Some(name) = new_name {
            if name.trim().is_empty() {
                return Err("Student name cannot be empty".to_string());
            }
            student.update_name(name);
        }

        if let Some(grade) = new_grade {
            if !(0.0..=100.0).contains(&grade) {
                return Err("Grade must be between 0.0 and 100.0".to_string());
            }
            student.update_grade(grade);
        }

        Ok(())
    }

    /// Update student status
    pub fn update_student_status(&mut self, id: u32, status: StudentStatus) -> Result<(), String> {
        let student = self.students.get_mut(&id)
            .ok_or_else(|| format!("Student with ID {} not found", id))?;

        student.set_status(status);
        Ok(())
    }

    /// Delete a student
    pub fn delete_student(&mut self, id: u32) -> Result<Student, String> {
        self.students.remove(&id)
            .ok_or_else(|| format!("Student with ID {} not found", id))
    }

    /// View a single student by ID
    pub fn view_student(&self, id: u32) -> Result<&Student, String> {
        self.students.get(&id)
            .ok_or_else(|| format!("Student with ID {} not found", id))
    }

    /// View all students
    pub fn view_all_students(&self) -> Vec<&Student> {
        let mut students: Vec<&Student> = self.students.values().collect();
        students.sort_by_key(|student| student.id);
        students
    }

    /// View only active students
    pub fn view_active_students(&self) -> Vec<&Student> {
        let mut active_students: Vec<&Student> = self.students.values()
            .filter(|student| student.is_active())
            .collect();
        active_students.sort_by_key(|student| student.id);
        active_students
    }

    /// View only inactive students
    pub fn view_inactive_students(&self) -> Vec<&Student> {
        let mut inactive_students: Vec<&Student> = self.students.values()
            .filter(|student| !student.is_active())
            .collect();
        inactive_students.sort_by_key(|student| student.id);
        inactive_students
    }

    /// Get total number of students
    pub fn total_students(&self) -> usize {
        self.students.len()
    }

    /// Get number of active students
    pub fn active_students_count(&self) -> usize {
        self.students.values()
            .filter(|student| student.is_active())
            .count()
    }

    /// Get number of inactive students
    pub fn inactive_students_count(&self) -> usize {
        self.students.values()
            .filter(|student| !student.is_active())
            .count()
    }

    /// Calculate average grade of all students
    pub fn average_grade(&self) -> Option<f64> {
        if self.students.is_empty() {
            return None;
        }

        let sum: f64 = self.students.values()
            .map(|student| student.grade)
            .sum();
        Some(sum / self.students.len() as f64)
    }

    /// Calculate average grade of active students only
    pub fn average_grade_active(&self) -> Option<f64> {
        let active_students: Vec<&Student> = self.students.values()
            .filter(|student| student.is_active())
            .collect();

        if active_students.is_empty() {
            return None;
        }

        let sum: f64 = active_students.iter()
            .map(|student| student.grade)
            .sum();
        Some(sum / active_students.len() as f64)
    }
}

impl Default for StudentManager {
    fn default() -> Self {
        Self::new()
    }
} 