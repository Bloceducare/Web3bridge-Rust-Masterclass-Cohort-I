// A class management system that has the name of the student, grade, enum that tracks if student is active or not.

// Have the following functions:
// - Function to register student
// - Edit
// - Update 
// - Delete functions
// - View function

#[derive(Debug)]
pub struct Student {
    name: String,
    grade: u8,
    active: Status,
}

pub struct Class {
    students: Vec<Student>,
}

#[derive(Debug)]
pub enum Status {
    Active,
    Inactive,
}

impl Class {
    fn new() -> Self {
        Class {
            students: Vec::new(),
        }
    }

    fn reg_student(&mut self, name: String, grade: u8, active: Status) {
        let student = Student { name, grade, active };
        self.students.push(student);
    }
    
    fn edit(&mut self, index: usize, name: String, grade: u8, active: Status) {
        if let Some(student) = self.students.get_mut(index) {
            student.name = name;
            student.grade = grade;
            student.active = active;
        }
    }

    fn delete(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        }
    }

    fn view(&self) {
        if self.students.is_empty() {
            println!("No students registered.");
            return;
        }
        
        for (index, student) in self.students.iter().enumerate() {
            println!("{}. Name: {}, Grade: {}, Status: {:?}", 
                     index + 1, student.name, student.grade, student.active);
        }
    }
}

fn main() {
    let mut class = Class::new();
    
    // Register students
    class.reg_student(String::from("Alice"), 10, Status::Active);
    class.reg_student(String::from("Bob"), 12, Status::Inactive);
    
    // View all students
    println!("All students:");
    class.view();
    
    // Edit a student (index 0 = Alice)
    class.edit(0, String::from("Alice Johnson"), 11, Status::Active);
    
    println!("\nAfter editing Alice:");
    class.view();
    
    // Delete a student (index 1 = Bob)
    class.delete(1);
    
    println!("\nAfter deleting Bob:");
    class.view();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_class() {
        let class = Class::new();
        assert_eq!(class.students.len(), 0);
    }

    #[test]
    fn test_register_student() {
        let mut class = Class::new();
        class.reg_student(String::from("Alice"), 10, Status::Active);
        
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].name, "Alice");
        assert_eq!(class.students[0].grade, 10);
        assert!(matches!(class.students[0].active, Status::Active));
    }

    #[test]
    fn test_register_multiple_students() {
        let mut class = Class::new();
        class.reg_student(String::from("Alice"), 10, Status::Active);
        class.reg_student(String::from("Bob"), 12, Status::Inactive);
        
        assert_eq!(class.students.len(), 2);
        assert_eq!(class.students[1].name, "Bob");
        assert_eq!(class.students[1].grade, 12);
        assert!(matches!(class.students[1].active, Status::Inactive));
    }

    #[test]
    fn test_edit_student() {
        let mut class = Class::new();
        class.reg_student(String::from("Alice"), 10, Status::Active);
        
        class.edit(0, String::from("Alice Johnson"), 11, Status::Inactive);
        
        assert_eq!(class.students[0].name, "Alice Johnson");
        assert_eq!(class.students[0].grade, 11);
        assert!(matches!(class.students[0].active, Status::Inactive));
    }

    #[test]
    fn test_edit_invalid_index() {
        let mut class = Class::new();
        class.reg_student(String::from("Alice"), 10, Status::Active);
        
        // Try to edit student at index 5 (doesn't exist)
        class.edit(5, String::from("Bob"), 12, Status::Active);
        
        // Original student should be unchanged
        assert_eq!(class.students[0].name, "Alice");
        assert_eq!(class.students[0].grade, 10);
        assert!(matches!(class.students[0].active, Status::Active));
    }

    #[test]
    fn test_delete_student() {
        let mut class = Class::new();
        class.reg_student(String::from("Alice"), 10, Status::Active);
        class.reg_student(String::from("Bob"), 12, Status::Inactive);
        
        assert_eq!(class.students.len(), 2);
        
        class.delete(0);
        
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].name, "Bob");
    }

    #[test]
    fn test_delete_invalid_index() {
        let mut class = Class::new();
        class.reg_student(String::from("Alice"), 10, Status::Active);
        
        // Try to delete student at index 5 (doesn't exist)
        class.delete(5);
        
        // Student should still be there
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].name, "Alice");
    }

    #[test]
    fn test_delete_all_students() {
        let mut class = Class::new();
        class.reg_student(String::from("Alice"), 10, Status::Active);
        class.reg_student(String::from("Bob"), 12, Status::Inactive);
        
        class.delete(0);
        class.delete(0); // Index 0 again because Alice was removed
        
        assert_eq!(class.students.len(), 0);
    }

    #[test]
    fn test_complex_operations() {
        let mut class = Class::new();
        
        // Register students
        class.reg_student(String::from("Alice"), 10, Status::Active);
        class.reg_student(String::from("Bob"), 12, Status::Inactive);
        class.reg_student(String::from("Charlie"), 11, Status::Active);
        
        // Edit middle student
        class.edit(1, String::from("Robert"), 13, Status::Active);
        
        // Delete first student
        class.delete(0);
        
        // Should have 2 students left: Robert and Charlie
        assert_eq!(class.students.len(), 2);
        assert_eq!(class.students[0].name, "Robert");
        assert_eq!(class.students[0].grade, 13);
        assert_eq!(class.students[1].name, "Charlie");
        assert_eq!(class.students[1].grade, 11);
    }
}