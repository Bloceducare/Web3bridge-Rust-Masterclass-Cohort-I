#[derive(Clone)]
enum Status {
    Active,
    InActive,
}

#[derive(Clone)]
enum Grade {
    A,
    B,
    C,
    D,
    E,
    F,
}
#[derive(Clone)]
struct Student {
    full_name: String,
    grade: Grade,
    status: Status,
}

struct Class {
    students: Vec<Student>,
}

impl Class {
    pub fn new() -> Self {
        Self {
            students: Vec::new(),
        }
    }

    pub fn register_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn edit_student(&mut self, index: usize, student: Student) -> bool {
        if let Some(prev_student) = self.students.get_mut(index) {
        *prev_student = student;
        true
    } else {
        false
    }
    }

    pub fn delete_student(&mut self, index: usize) -> bool{
        if let Some(student) = self.students.get(index) {
            self.students.remove(index);
            true
        } else {
        false
    }
    }


    pub fn view_student_by_index(&self, index:usize) -> &Student {
        &self.students.get(index).unwrap()
    }

    pub fn view_all_students(&self) -> Vec<Student> {
        self.students.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registeration(){
        let mut class = Class::new();
        assert!(class.students.len() == 0);
        
        let student = Student {
            full_name: "Joseph".to_string(),
            grade: Grade::A,
            status: Status::Active,
        };

        class.register_student(student);
        assert!(class.students.len() == 1);
    }

    #[test]
    fn test_edit(){
        let mut class = Class::new();
        assert!(class.students.len() == 0);
        
        let student = Student {
            full_name: "Joseph".to_string(),
            grade: Grade::A,
            status: Status::Active,
        };

        class.register_student(student);
        assert!(class.students.len() == 1);

        let new_student = Student {
            full_name: "Leo".to_string(),
            grade: Grade::B,
            status: Status::InActive,
        };

        let updated = class.edit_student(0, new_student);
        assert!(class.students.len() == 1);
        assert!(updated == true);
    }

    #[test]
    fn test_delete(){
        let mut class = Class::new();
        assert!(class.students.len() == 0);
        
        let student = Student {
            full_name: "Joseph".to_string(),
            grade: Grade::A,
            status: Status::Active,
        };

        class.register_student(student);
        assert!(class.students.len() == 1);

        let deleted = class.delete_student(0);
        assert!(deleted == true);
    }

    #[test]
    fn test_view_student(){
        let mut class = Class::new();
        assert!(class.students.len() == 0);
        
        let student = Student {
            full_name: "Joseph".to_string(),
            grade: Grade::A,
            status: Status::Active,
        };

        class.register_student(student);
        assert!(class.students.len() == 1);

        let student_result = class.view_student_by_index(0);
        assert!(student_result.full_name == "Joseph".to_string());
    }

    #[test]
    fn test_view_all(){
        let mut class = Class::new();
        assert!(class.students.len() == 0);
        
        let student = Student {
            full_name: "Joseph".to_string(),
            grade: Grade::A,
            status: Status::Active,
        };

        class.register_student(student);
        assert!(class.students.len() == 1);

        let new_student = Student {
            full_name: "Leo".to_string(),
            grade: Grade::B,
            status: Status::InActive,
        };

        class.register_student(new_student);
        assert!(class.students.len() == 2);

        let students = class.view_all_students();
        assert!(students.len() == 2);
    }

}