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
    pub name: String,
    pub grade: u8,
    pub status: Status,
}

pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Active,
    Inactive,
}

impl Student {
    pub fn new(name: String, grade: u8, status: Status) -> Self {
        Self { name, grade, status }
    }

    pub fn edit(&mut self, name: String, grade: u8, status: Status) {
        self.name = name;
        self.grade = grade;
        self.status = status;
    }

    pub fn update(&mut self, name: String, grade: u8, status: Status) {
        self.name = name;
        self.grade = grade;
        self.status = status;
    }

    pub fn delete(&mut self) {
        self.status = Status::Inactive;
    }

    pub fn view(&self) {
        println!("Name: {}", self.name);
        println!("Grade: {}", self.grade);
        println!("Status: {:?}", self.status);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_student() {
        let student = Student::new("Ebele Okolo".to_string(), 12, Status::Active);
        assert_eq!(student.name, "Ebele Okolo");
        assert_eq!(student.grade, 12);
        assert_eq!(student.status, Status::Active);
    }

    #[test]
    fn test_edit_student() {
        let mut student = Student::new("Ebele Okolo".to_string(), 12, Status::Active);
        student.edit("Lynda Okolo".to_string(), 12, Status::Active);
        assert_eq!(student.name, "Lynda Okolo");
        assert_eq!(student.grade, 12);
        assert_eq!(student.status, Status::Active);
    }

    #[test]
    fn test_update_student() {
        let mut student = Student::new("Ebele Okolo".to_string(), 12, Status::Active);
        student.update("Lynda Okolo".to_string(), 12, Status::Active);
        assert_eq!(student.name, "Lynda Okolo");
        assert_eq!(student.grade, 12);
        assert_eq!(student.status, Status::Active);
    }

    #[test]
    fn test_delete_student() {
        let mut student = Student::new("Ebele Okolo".to_string(), 12, Status::Active);
        student.delete();
        assert_eq!(student.status, Status::Inactive);
    }
}