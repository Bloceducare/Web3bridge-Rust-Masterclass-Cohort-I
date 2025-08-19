use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum StudentStatus {
    Active,
    Inactive,
}

impl fmt::Display for StudentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StudentStatus::Active => write!(f, "Active"),
            StudentStatus::Inactive => write!(f, "Inactive"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grade: String,
    pub status: StudentStatus,
}

impl Student {
    pub fn new(id: u32, name: String, grade: String) -> Self {
        Student {
            id,
            name,
            grade,
            status: StudentStatus::Active,
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}, Name: {}, Grade: {}, Status: {}",
            self.id, self.name, self.grade, self.status
        )
    }
}

#[derive(Debug)]
pub enum ClassManagementError {
    StudentNotFound,
    StudentAlreadyExists,
    InvalidGrade,
    InvalidName,
}

impl fmt::Display for ClassManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClassManagementError::StudentNotFound => write!(f, "Student not found"),
            ClassManagementError::StudentAlreadyExists => write!(f, "Student already exists"),
            ClassManagementError::InvalidGrade => write!(f, "Invalid grade provided"),
            ClassManagementError::InvalidName => write!(f, "Invalid name provided"),
        }
    }
}

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

    // Function to register a new student
    pub fn register_student(
        &mut self,
        name: String,
        grade: String,
    ) -> Result<u32, ClassManagementError> {
        // Validate input
        if name.trim().is_empty() {
            return Err(ClassManagementError::InvalidName);
        }

        if grade.trim().is_empty() {
            return Err(ClassManagementError::InvalidGrade);
        }

        let student_id = self.next_id;
        let student = Student::new(
            student_id,
            name.trim().to_string(),
            grade.trim().to_string(),
        );

        self.students.insert(student_id, student);
        self.next_id += 1;

        Ok(student_id)
    }

    // Function to edit student information
    pub fn edit_student(
        &mut self,
        id: u32,
        name: Option<String>,
        grade: Option<String>,
        status: Option<StudentStatus>,
    ) -> Result<(), ClassManagementError> {
        let student = self
            .students
            .get_mut(&id)
            .ok_or(ClassManagementError::StudentNotFound)?;

        if let Some(new_name) = name {
            if new_name.trim().is_empty() {
                return Err(ClassManagementError::InvalidName);
            }
            student.name = new_name.trim().to_string();
        }

        if let Some(new_grade) = grade {
            if new_grade.trim().is_empty() {
                return Err(ClassManagementError::InvalidGrade);
            }
            student.grade = new_grade.trim().to_string();
        }

        if let Some(new_status) = status {
            student.status = new_status;
        }

        Ok(())
    }

    // Function to update student status
    pub fn update_student_status(
        &mut self,
        id: u32,
        status: StudentStatus,
    ) -> Result<(), ClassManagementError> {
        let student = self
            .students
            .get_mut(&id)
            .ok_or(ClassManagementError::StudentNotFound)?;
        student.status = status;
        Ok(())
    }

    // Function to delete a student
    pub fn delete_student(&mut self, id: u32) -> Result<Student, ClassManagementError> {
        self.students
            .remove(&id)
            .ok_or(ClassManagementError::StudentNotFound)
    }

    // Function to view a specific student
    pub fn view_student(&self, id: u32) -> Result<&Student, ClassManagementError> {
        self.students
            .get(&id)
            .ok_or(ClassManagementError::StudentNotFound)
    }

    // Function to view all students
    pub fn view_all_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }

    // Function to view students by status
    pub fn view_students_by_status(&self, status: StudentStatus) -> Vec<&Student> {
        self.students
            .values()
            .filter(|student| student.status == status)
            .collect()
    }

    // Function to get the total number of students
    pub fn student_count(&self) -> usize {
        self.students.len()
    }

    // Function to check if a student exists
    pub fn student_exists(&self, id: u32) -> bool {
        self.students.contains_key(&id)
    }
}

impl Default for ClassManagementSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_system() {
        let system = ClassManagementSystem::new();
        assert_eq!(system.student_count(), 0);
        assert_eq!(system.next_id, 1);
    }

    #[test]
    fn test_register_student() {
        let mut system = ClassManagementSystem::new();

        let result = system.register_student("John Doe".to_string(), "10th".to_string());
        assert!(result.is_ok());

        let student_id = result.unwrap();
        assert_eq!(student_id, 1);
        assert_eq!(system.student_count(), 1);
        assert!(system.student_exists(student_id));
    }

    #[test]
    fn test_register_student_invalid_name() {
        let mut system = ClassManagementSystem::new();

        let result = system.register_student("".to_string(), "10th".to_string());
        assert!(result.is_err());

        match result.unwrap_err() {
            ClassManagementError::InvalidName => (),
            _ => panic!("Expected InvalidName error"),
        }
    }

    #[test]
    fn test_register_student_invalid_grade() {
        let mut system = ClassManagementSystem::new();

        let result = system.register_student("John Doe".to_string(), "".to_string());
        assert!(result.is_err());

        match result.unwrap_err() {
            ClassManagementError::InvalidGrade => (),
            _ => panic!("Expected InvalidGrade error"),
        }
    }

    #[test]
    fn test_view_student() {
        let mut system = ClassManagementSystem::new();
        let student_id = system
            .register_student("Jane Smith".to_string(), "9th".to_string())
            .unwrap();

        let result = system.view_student(student_id);
        assert!(result.is_ok());

        let student = result.unwrap();
        assert_eq!(student.name, "Jane Smith");
        assert_eq!(student.grade, "9th");
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_view_student_not_found() {
        let system = ClassManagementSystem::new();

        let result = system.view_student(999);
        assert!(result.is_err());

        match result.unwrap_err() {
            ClassManagementError::StudentNotFound => (),
            _ => panic!("Expected StudentNotFound error"),
        }
    }

    #[test]
    fn test_edit_student() {
        let mut system = ClassManagementSystem::new();
        let student_id = system
            .register_student("Bob Johnson".to_string(), "11th".to_string())
            .unwrap();

        let result = system.edit_student(
            student_id,
            Some("Robert Johnson".to_string()),
            Some("12th".to_string()),
            Some(StudentStatus::Inactive),
        );
        assert!(result.is_ok());

        let student = system.view_student(student_id).unwrap();
        assert_eq!(student.name, "Robert Johnson");
        assert_eq!(student.grade, "12th");
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_edit_student_partial() {
        let mut system = ClassManagementSystem::new();
        let student_id = system
            .register_student("Alice Brown".to_string(), "10th".to_string())
            .unwrap();

        let result = system.edit_student(student_id, None, Some("11th".to_string()), None);
        assert!(result.is_ok());

        let student = system.view_student(student_id).unwrap();
        assert_eq!(student.name, "Alice Brown");
        assert_eq!(student.grade, "11th");
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_edit_student_not_found() {
        let mut system = ClassManagementSystem::new();

        let result = system.edit_student(999, Some("Test".to_string()), None, None);
        assert!(result.is_err());

        match result.unwrap_err() {
            ClassManagementError::StudentNotFound => (),
            _ => panic!("Expected StudentNotFound error"),
        }
    }

    #[test]
    fn test_update_student_status() {
        let mut system = ClassManagementSystem::new();
        let student_id = system
            .register_student("Charlie Davis".to_string(), "9th".to_string())
            .unwrap();

        let result = system.update_student_status(student_id, StudentStatus::Inactive);
        assert!(result.is_ok());

        let student = system.view_student(student_id).unwrap();
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_delete_student() {
        let mut system = ClassManagementSystem::new();
        let student_id = system
            .register_student("Diana Evans".to_string(), "8th".to_string())
            .unwrap();

        assert_eq!(system.student_count(), 1);

        let result = system.delete_student(student_id);
        assert!(result.is_ok());

        let deleted_student = result.unwrap();
        assert_eq!(deleted_student.name, "Diana Evans");
        assert_eq!(system.student_count(), 0);
        assert!(!system.student_exists(student_id));
    }

    #[test]
    fn test_delete_student_not_found() {
        let mut system = ClassManagementSystem::new();

        let result = system.delete_student(999);
        assert!(result.is_err());

        match result.unwrap_err() {
            ClassManagementError::StudentNotFound => (),
            _ => panic!("Expected StudentNotFound error"),
        }
    }

    #[test]
    fn test_view_all_students() {
        let mut system = ClassManagementSystem::new();

        system
            .register_student("Student 1".to_string(), "9th".to_string())
            .unwrap();
        system
            .register_student("Student 2".to_string(), "10th".to_string())
            .unwrap();
        system
            .register_student("Student 3".to_string(), "11th".to_string())
            .unwrap();

        let all_students = system.view_all_students();
        assert_eq!(all_students.len(), 3);
    }

    #[test]
    fn test_view_students_by_status() {
        let mut system = ClassManagementSystem::new();

        let id1 = system
            .register_student("Active Student".to_string(), "9th".to_string())
            .unwrap();
        let id2 = system
            .register_student("Inactive Student".to_string(), "10th".to_string())
            .unwrap();
        let id3 = system
            .register_student("Another Active".to_string(), "11th".to_string())
            .unwrap();

        system
            .update_student_status(id2, StudentStatus::Inactive)
            .unwrap();

        let active_students = system.view_students_by_status(StudentStatus::Active);
        let inactive_students = system.view_students_by_status(StudentStatus::Inactive);

        assert_eq!(active_students.len(), 2);
        assert_eq!(inactive_students.len(), 1);
        assert_eq!(inactive_students[0].name, "Inactive Student");
    }

    #[test]
    fn test_student_display() {
        let student = Student::new(1, "Test Student".to_string(), "10th".to_string());
        let display_string = format!("{}", student);
        assert_eq!(
            display_string,
            "ID: 1, Name: Test Student, Grade: 10th, Status: Active"
        );
    }

    #[test]
    fn test_student_status_display() {
        assert_eq!(format!("{}", StudentStatus::Active), "Active");
        assert_eq!(format!("{}", StudentStatus::Inactive), "Inactive");
    }

    #[test]
    fn test_error_display() {
        let error = ClassManagementError::StudentNotFound;
        assert_eq!(format!("{}", error), "Student not found");
    }

    #[test]
    fn test_multiple_registrations() {
        let mut system = ClassManagementSystem::new();

        let id1 = system
            .register_student("Student 1".to_string(), "9th".to_string())
            .unwrap();
        let id2 = system
            .register_student("Student 2".to_string(), "10th".to_string())
            .unwrap();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(system.next_id, 3);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut system = ClassManagementSystem::new();

        let id = system
            .register_student("  John Doe  ".to_string(), "  10th  ".to_string())
            .unwrap();
        let student = system.view_student(id).unwrap();

        assert_eq!(student.name, "John Doe");
        assert_eq!(student.grade, "10th");
    }
}
