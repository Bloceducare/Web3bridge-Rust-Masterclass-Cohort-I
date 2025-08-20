#[derive(Debug, Clone, PartialEq)]
enum StudentStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    grade: u32,
    status: StudentStatus,
}

#[derive(Debug)]
struct Catalogue {
    students: Vec<Student>,
}

impl Catalogue {
    fn new() -> Self {
        Catalogue {
            students: Vec::new(),
        }
    }

    fn register_student(&mut self, id: u32, name: String, grade: u32) {
        let student = Student {
            id,
            name,
            grade,
            status: StudentStatus::Inactive,
        };
        self.students.push(student);
    }

    fn edit_student(&mut self, id: u32, name: &str, grade: u32) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.name = name.to_string();
            student.grade = grade;
        }
    }


    /*
    fn update_student(&mut self, id: u32, new_name: String) -> bool {
    if let Some(student) = self.data.iter_mut().find(|s| s.id == id) {
        student.name = new_name;
        true
    } else {
        false
    }
}
    */

    fn update_student(
        &mut self,
        id: u32,
        new_name: Option<&str>,
        new_grade: Option<&u32>,
        new_status: Option<StudentStatus>,
    ) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            if let Some(name) = new_name {
                student.name = name.to_string();
            }
            if let Some(grade) = new_grade {
                student.grade = *grade;
            }
            if let Some(status) = new_status {
                student.status = status;
            }
        }
    }

    fn update_student_status(&mut self, id:u32, status: StudentStatus) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.status = status
        }
        
    }

    fn delete_student(&mut self, id: u32) {
        self.students.retain(|s| s.id != id);
    }

    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == id)
    }
}

// Optional main
fn main() {
    let mut catalogue = Catalogue::new();
    catalogue.register_student(1, "Alice".to_string(), 85);
    catalogue.register_student(2, "Bob".to_string(), 78);
    catalogue.edit_student(1, "Alicia", 90);
    catalogue.update_student(2, Some("Robert"), Some(&95), Some(StudentStatus::Active));
    catalogue.delete_student(1);
    println!("Students in catalogue:");
    for s in catalogue.students {
        println!("{:?}", s);
    }
}


//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut catalogue = Catalogue::new();
        catalogue.register_student(1, "Test".to_string(), 50);
        let student = catalogue.get_student(1).unwrap();
        assert_eq!(student.name, "Test");
        assert_eq!(student.grade, 50);
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_edit_student() {
        let mut catalogue = Catalogue::new();
        catalogue.register_student(1, "John".to_string(), 70);
        catalogue.edit_student(1, "Johnny", 75);
        let student = catalogue.get_student(1).unwrap();
        assert_eq!(student.name, "Johnny");
        assert_eq!(student.grade, 75);
    }

    #[test]
    fn test_update_student() {
        let mut catalogue = Catalogue::new();
        catalogue.register_student(1, "Jane".to_string(), 60);
        catalogue.update_student(1, Some("Janet"), Some(&80), Some(StudentStatus::Active));
        let student = catalogue.get_student(1).unwrap();
        assert_eq!(student.name, "Janet");
        assert_eq!(student.grade, 80);
        assert_eq!(student.status, Active::Active);
    }

    #[test]
    fn test_update_student_status(){

    }

    #[test]
    fn test_delete_student() {
        let mut catalogue = Catalogue::new();
        catalogue.register_student(1, "Mark".to_string(), 65);
        assert!(catalogue.get_student(1).is_some());
        catalogue.delete_student(1);
        assert!(catalogue.get_student(1).is_none());
    }
}
