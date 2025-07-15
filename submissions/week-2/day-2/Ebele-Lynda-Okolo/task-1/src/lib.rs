// A class management system that has the name of the student, grade, enum that tracks if student is active or not.

// Have the following functions:
// - Function to register student
// - Edit
// - Update 
// - Delete functions
// - View function
// Test for all the function

#[derive(Debug)]
pub struct Student {
    name: String,
    grade: u8,
    is_active: bool,
}

pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}

pub struct Student {
    pub student_db: Vec<Student>
}

impl Student {
    pub fn new(name: String, grade: u8, is_active: bool) -> Self {
        Self { name, grade, is_active }
    }

    pub fn edit(&mut self, name: String, grade: u8, is_active: bool) {
        self.name = name;
        self.grade = grade;
        self.is_active = is_active;
    }

    pub fn update(&mut self, name: String, grade: u8, is_active: bool) {
        self.name = name;
        self.grade = grade;
        self.is_active = is_active;
    }

    pub fn delete(&mut self) {
        self.is_active = false;
    }

    pub fn view(&self) {
        println!("Name: {}", self.name);
        println!("Grade: {}", self.grade);
        println!("Active: {}", self.is_active);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_student() {
        let student = Student::new("Ebele Okolo".to_string(), 12, true);
        assert_eq!(student.name, "Ebele Okolo");
        assert_eq!(student.grade, 12);
        assert_eq!(student.is_active, true);
    }

    #[test]
    fn test_edit_student() {
        let mut student = Student::new("Ebele Okolo".to_string(), 12, true);
        student.edit("Lynda Okolo".to_string(), 12, true);
        assert_eq!(student.name, "Lynda Okolo");
        assert_eq!(student.grade, 12);
        assert_eq!(student.is_active, true);
    }

    #[test]
    fn test_update_student() {
        let mut student = Student::new("Ebele Okolo".to_string(), 12, true);
        student.update("Lynda Okolo".to_string(), 12, true);
        assert_eq!(student.name, "Lynda Okolo");
        assert_eq!(student.grade, 12);
        assert_eq!(student.is_active, true);
    }

    #[test]
    fn test_delete_student() {
        let mut student = Student::new("Ebele Okolo".to_string(), 12, true);
        student.delete();
        assert_eq!(student.is_active, false);
    }
}