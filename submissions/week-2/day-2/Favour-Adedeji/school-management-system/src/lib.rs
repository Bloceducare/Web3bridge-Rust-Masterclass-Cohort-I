#[derive(Clone, Debug, PartialEq)]
pub enum Active {
    ACTIVE,
    INACTIVE,
}

#[derive(Clone, Debug)]
pub struct Student {
    pub id: usize,
    pub name: String,
    pub grade: String,
    pub status: Active,
}

pub struct School {
    pub students: Vec<Student>,
}

impl School {
    pub fn initialize() -> Self {
        Self {
            students: Vec::new(),
        }
    }

    pub fn create_student(&mut self, mut student: Student) {
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

    pub fn update_student(&mut self, id: usize, student: Student) {
        if let Some(st) = self.students.get_mut(id) {
            st.name = student.name;
            st.status = student.status;
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

    pub fn update_status(&mut self, id: usize) {
        for mut student in &mut self.students {
            if student.id == id {
                match student.status {
                    Active::ACTIVE => {
                        println!("Student with id: {}, is active", id);
                        student.status = Active::INACTIVE;
                        println!("Student status is now {:?}", student.status);
                    }
                    Active::INACTIVE => {
                        println!("Student with id: {}, is inactive", id);
                        student.status = Active::ACTIVE;
                        println!("Student status is now {:?}", student.status);
                    }
                }
                break;
            }
        }
    }
    pub fn change_status(&mut self, id: usize, status: Active) {
        if let Some(target) = self.students.iter_mut().find(|student| student.id == id) {
            target.status = status;
        } else {
            println!("Student with id {}, not found", id);
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
            id: 1,
            name: "John".to_string(),
            grade: String::from("Class 6"),
            status: Active::ACTIVE,
        };

        school.create_student(student);
        assert!(school.students.len() == 1);
    }

    #[test]
    fn test_get_students() {
        let mut school = School::initialize();
        assert!(school.students.len() == 0);

        let student = Student {
            id: 1,
            name: "Rick".to_string(),
            grade: "class 5".to_string(),
            status: Active::ACTIVE,
        };

        let student2 = Student {
            id: 2,
            name: "John".to_string(),
            grade: "class 6".to_string(),
            status: Active::INACTIVE,
        };

        school.create_student(student);
        school.create_student(student2);

        let students = school.get_students();

        assert_eq!(students.len(), 2);
    }

    #[test]
    fn test_fetch_student() {
        let mut school = School::initialize();
        assert!(school.students.len() == 0);

        let student = Student {
            id: 1,
            name: "Rick".to_string(),
            grade: "class 5".to_string(),
            status: Active::ACTIVE,
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
            id: 1,
            name: "John".to_string(),
            grade: "class 6".to_string(),
            status: Active::ACTIVE,
        };

        let updated_student = Student {
            id: 1,
            name: "Rick".to_string(),
            grade: "class 6".to_string(),
            status: Active::ACTIVE,
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
            id: 1,
            name: "John".to_string(),
            grade: "class 6".to_string(),
            status: Active::ACTIVE,
        };
        let student2 = Student {
            id: 2,
            name: "John".to_string(),
            grade: "class 6".to_string(),
            status: Active::ACTIVE,
        };
        let student3 = Student {
            id: 3,
            name: "John".to_string(),
            grade: "class 6".to_string(),
            status: Active::ACTIVE,
        };

        school.create_student(student);
        school.create_student(student2);
        school.create_student(student3);

        assert_eq!(school.students.len(), 3);
        school.delete_student(1);
        assert_eq!(school.students.len(), 2);
    }

    #[test]
    fn test_update_status() {
        let mut school = School::initialize();
        let student = Student {
            id: 1,
            name: "Rick".to_string(),
            grade: "class 5".to_string(),
            status: Active::ACTIVE,
        };

        school.create_student(student);
        school.update_status(1);

        let student = school.fetch_student(0);
        assert_eq!(student.status, Active::INACTIVE);

        school.update_status(1);
        let student = school.fetch_student(0);
        assert_eq!(student.status, Active::ACTIVE);
    }

    #[test]
    fn test_change_status() {
        let mut school = School::initialize();
        let student = Student {
            id: 1,
            name: "Rick".to_string(),
            grade: "class 5".to_string(),
            status: Active::ACTIVE,
        };

        school.create_student(student);

        school.change_status(1, Active::INACTIVE);
        let student = school.fetch_student(0);
        assert_eq!(student.status, Active::INACTIVE);

        school.change_status(1, Active::ACTIVE);
        let student = school.fetch_student(0);
        assert_eq!(student.status, Active::ACTIVE);
    }
}
