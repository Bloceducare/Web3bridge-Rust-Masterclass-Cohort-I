// School management system
// Takes in name, grade
// Enum (active, inactive)

// Implementation function
// Register
// Update
// Delete
// View

#[derive(Clone, Debug)]
pub struct Student {
    pub name: String,
    pub grade: u8,
    pub status: Status,
}

#[derive(Clone, Debug)]
pub enum Status {
    Active,
    Inactive,
}

pub struct School {
    pub studentData: Vec<Student>,
}

impl School {
    pub fn init() -> School {
        School {
            studentData: Vec::new(),
        }
    }

    pub fn register_student(&mut self, name: String, grade: u8) {
        let student = Student {
            name,
            grade,
            status: Status::Active,
        };

        self.studentData.push(student);
    }
    pub fn update_student(&mut self, name: String, grade: u8, status: Status, index: usize) {
        let update = &mut self.studentData[index];
        update.name = name.to_string();
        update.grade = grade;
        update.status = status;
    }

    pub fn delete_studentData(&mut self, index: usize) {
        self.studentData.remove(index);
    }
    pub fn view_studentData(&self, index: usize) -> &Student {
        self.studentData.get(index).unwrap()
    }

    pub fn view_all_studentData(&self) -> Vec<Student> {
        self.studentData.to_vec()
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut school = School::init();
        assert_eq!(school.studentData.len(), 0);

        school.register_student("John".to_string(), 10);
        
        assert_eq!(school.studentData.len(), 1);
        assert_eq!(school.studentData[0].name, "John");
        assert_eq!(school.studentData[0].grade, 10);
    }

    #[test]
    fn test_register_multiple_students() {
        let mut school = School::init();
        
        school.register_student("Alice".to_string(), 9);
        school.register_student("Bob".to_string(), 11);
        school.register_student("Josh".to_string(), 12);

        assert_eq!(school.studentData.len(), 3);
        assert_eq!(school.studentData[0].name, "Alice");
        assert_eq!(school.studentData[1].name, "Bob");
        assert_eq!(school.studentData[2].name, "Josh");
    }

    #[test]
    fn test_update_student() {
        let mut school = School::init();
        
        // Register a student first
        school.register_student("Josh".to_string(), 9);
        
        // Verify initial state
        assert_eq!(school.studentData[0].name, "Josh");
        assert_eq!(school.studentData[0].grade, 9);
   
        // Update the student
        school.update_student("James".to_string(), 10, Status::Inactive, 0);
        
        // Verify updated state
        assert_eq!(school.studentData[0].name, "James");
        assert_eq!(school.studentData[0].grade, 10);
    }

}