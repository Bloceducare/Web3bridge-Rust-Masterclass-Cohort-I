#[derive(Clone, Debug, PartialEq)]
struct Student {
    name: String,
    grade: u8,
    status: Status,
}

pub struct School {
    pub students: Vec<Student>
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Active,
    Inactive,
}

impl School {
    fn new() -> Self {
        School { students: Vec::new() }
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn update_student(&mut self, index: usize, student: Student) {
        if index < self.students.len() {
            self.students[index] = student;
        }
    }

    fn remove_student(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        }
    }

    fn get_students(&mut self, index: usize) -> &mut Student {
        self.students.get_mut(index).expect("Index out of bounds")
    }

    fn set_status(&mut self, index: usize, new_status: &Status) {
        let student = self.get_students(index);
        student.status = new_status.clone();
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_student() {
        let mut school = School::new();
        let student = Student { 
            name: String::from("Alice"),
            grade: 10,
            status: Status::Active,
        };
        school.add_student(student);
        assert_eq!(school.students.len(), 1);
        assert_eq!(school.get_students(0).name, "Alice");
    }

    #[test]
    fn test_update_student() {
        let mut school = School::new();
        let student = Student { 
            name: String::from("Alice"),
            grade: 10,
            status: Status::Active,
        };
        school.add_student(student);
        let updated_student = Student {
            name: String::from("Alice"),
            grade: 11,
            status: Status::Inactive,
        };
        school.update_student(0, updated_student);
        assert_eq!(school.get_students(0).grade, 11);
    }

    #[test]
    fn test_remove_student() {
        let mut school = School::new();
        let student = Student { 
            name: String::from("Alice"),
            grade: 10,
            status: Status::Active,
        };
        school.add_student(student);
        school.remove_student(0);
        assert!(school.students.is_empty());
    }

    #[test]
    fn test_set_status() {
        let mut school = School::new();
        let student = Student {
            name: String::from("Alice"),
            grade: 10,
            status: Status::Active,
        };
        school.add_student(student);
        school.set_status(0, &Status::Inactive);
        assert_eq!(school.get_students(0).status, Status::Inactive);
    }

    #[test]
    fn test_get_students() {
        let mut school = School::new();
        let student = Student {
            name: String::from("Alice"),
            grade: 10,
            status: Status::Active,
        };
        school.add_student(student);
        let retrieved_student = school.get_students(0);
        assert_eq!(retrieved_student.name, "Alice");
        assert_eq!(retrieved_student.grade, 10);
        assert_eq!(retrieved_student.status, Status::Active);
        assert_eq!(retrieved_student.status, Status::Active);
    }   
}

