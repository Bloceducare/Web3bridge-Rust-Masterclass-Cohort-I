#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub grade: u8,
    pub status: Status,
}

pub struct ClassManager {
    pub students: Vec<Student>,
}

impl ClassManager {
    pub fn new() -> Self {
        ClassManager {
            students: Vec::new(),
        }
    }

    // Register student
    pub fn register_student(&mut self, name: String, grade: u8, status: Status) {
        let student = Student { name, grade, status };
        self.students.push(student);
    }

    // Edit student's name and grade
    pub fn edit_student(&mut self, index: usize, new_name: String, new_grade: u8) {
        if let Some(student) = self.students.get_mut(index) {
            student.name = new_name;
            student.grade = new_grade;
        } else {
            println!("No student found at index {}", index);
        }
    }

    // Update status only
    pub fn update_status(&mut self, index: usize, new_status: Status) {
        if let Some(student) = self.students.get_mut(index) {
            student.status = new_status;
        } else {
            println!("No student found at index {}", index);
        }
    }

    // Delete student
    pub fn delete_student(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        } else {
            println!("Invalid index, cannot delete.");
        }
    }

    // View all students
    pub fn view_students(&self) {
        if self.students.is_empty() {
            println!("No students registered.");
        } else {
            for (i, student) in self.students.iter().enumerate() {
                println!("{}: {:?}, Grade: {}, Status: {:?}", i, student.name, student.grade, student.status);
            }
        }
    }
}

fn main() {
    let mut class = ClassManager::new();

    // Register students
    class.register_student("Alice".to_string(), 10, Status::Active);
    class.register_student("Bob".to_string(), 9, Status::Inactive);

    // View all
    class.view_students();

    // Edit student
    class.edit_student(0, "Alicia".to_string(), 11);

    // Update status
    class.update_status(1, Status::Active);

    // Delete student
    class.delete_student(0);

    // Final state
    class.view_students();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut class = ClassManager::new();
        class.register_student("John".to_string(), 8, Status::Active);
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].name, "John");
        assert_eq!(class.students[0].grade, 8);
        assert_eq!(class.students[0].status, Status::Active);
    }

    #[test]
    fn test_edit_student() {
        let mut class = ClassManager::new();
        class.register_student("Anna".to_string(), 9, Status::Active);
        class.edit_student(0, "Annabelle".to_string(), 10);
        assert_eq!(class.students[0].name, "Annabelle");
        assert_eq!(class.students[0].grade, 10);
    }

    #[test]
    fn test_update_status() {
        let mut class = ClassManager::new();
        class.register_student("Leo".to_string(), 7, Status::Inactive);
        class.update_status(0, Status::Active);
        assert_eq!(class.students[0].status, Status::Active);
    }

    #[test]
    fn test_delete_student() {
        let mut class = ClassManager::new();
        class.register_student("Mike".to_string(), 10, Status::Active);
        class.register_student("Jane".to_string(), 9, Status::Inactive);
        class.delete_student(0);
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].name, "Jane");
    }

    #[test]
    fn test_view_students_output() {
        let mut class = ClassManager::new();
        class.register_student("Sam".to_string(), 6, Status::Active);

        // Just ensuring no panic occurs while viewing students
        class.view_students();
    }
}

