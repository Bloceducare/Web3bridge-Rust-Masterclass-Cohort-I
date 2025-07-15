#[derive(Clone, Debug)]
pub enum Active {
    TRUE,
    FALSE,
}

#[derive(Clone, Debug)]
pub struct Student {
    pub name: String,
    pub grade: String,
    pub active: Active,
}

pub struct School {
    pub students: Vec<Student>
}

impl School {
    pub fn initialize() -> Self {
        Self {
            students: Vec::new()
        }
    }

    pub fn create_student(&mut self, student: Student) {
        self.students.push(student);
        println!("Student created successfully");
    }

    pub fn get_students(&self) -> Vec<Student> {
        self.students.to_vec()
    }

    pub fn fetch_student(&self, index: usize) -> &Student {
        if self.students.len() > index {
            &self.students[index]
        } else {
            panic!("Out of bound");
        }
    }

    pub fn update_student(&mut self, id: usize, student: Student){
        if let Some(st) = self.students.get_mut(id) {
            st.name = student.name;
            st.active = student.active;
            st.grade = student.grade;
        } else {
            panic!("Student with id {} does not exist", id);
        }
    }

    pub fn delete_student(&mut self, id: usize) {
        if id < self.students.len() {
            self.students.remove(id);
            println!("Student deleted successfully");
        } else {
            panic!("Student with id {} does not exist", id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_student() {
        let mut school = School::initialize();
        assert!(school.students.len() == 0);

        let student = Student {
            name: "John".to_string(),
            grade: String::from("Class 6"),
            active: Active::TRUE,
        };

        school.create_student(student);
        assert!(school.students.len() == 1);
    }

    #[test]
    fn test_get_students(){
        let mut school = School::initialize();
        assert!(school.students.len() == 0);

        let student = Student {
        name: "Rick".to_string(),
        grade: "class 5".to_string(),
        active: Active::TRUE
        };

        let student2 = Student {
            name: "John".to_string(),
            grade: "class 6".to_string(),
            active: Active::FALSE
        };

        school.create_student(student);
        school.create_student(student2);

        let students = school.get_students();

        assert_eq!(students.len(), 2);
    }

    #[test]
    fn test_fetch_student(){
        let mut school = School::initialize();
        assert!(school.students.len() == 0);

        let student = Student {
          name: "Rick".to_string(),
          grade: "class 5".to_string(),
          active: Active::TRUE
        };

        school.create_student(student);

        let student = school.fetch_student(0);

        assert_eq!(student.name, "Rick".to_string());
        assert_eq!(student.grade, "class 5".to_string());
    }

    #[test]
    fn test_update_student() {
        let mut school = School::initialize();

        let student = Student {
          name: "John".to_string(),
          grade: "class 6".to_string(),
          active: Active::TRUE
        };

        let updated_student = Student {
          name: "Rick".to_string(),
          grade: "class 6".to_string(),
          active: Active::TRUE
        };

        school.create_student(student);

        school.update_student(0, updated_student);
        let student = school.fetch_student(0);

        assert_eq!(student.name, "Rick".to_string());
    }

    #[test]
    fn test_delete_student() {
        let mut school = School::initialize();
        let student = Student {
            name: "John".to_string(),
            grade: "class 6".to_string(),
            active: Active::TRUE
        };
        let student2 = Student {
            name: "John".to_string(),
            grade: "class 6".to_string(),
            active: Active::TRUE
        };
        let student3 = Student {
            name: "John".to_string(),
            grade: "class 6".to_string(),
            active: Active::TRUE
        };

        school.create_student(student);
        school.create_student(student2);
        school.create_student(student3);

        assert_eq!(school.students.len(), 3);
        school.delete_student(1);
        assert_eq!(school.students.len(), 2);
    }
}
