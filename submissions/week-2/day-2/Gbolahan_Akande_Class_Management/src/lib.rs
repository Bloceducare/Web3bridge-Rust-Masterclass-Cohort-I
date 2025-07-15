#[derive(Clone, Debug)]
pub enum StudentStatus {
    Active,
    Inactive,
}

#[derive(Clone, Debug)]
pub struct Student {
    pub name: String,
    pub grade: i32,
    pub status: StudentStatus,
}

pub struct Management {
    pub students: Vec<Student>
}

impl Management {
    pub fn initialize() -> Management {
        Management {
            students: Vec::new()
        }
    }

    pub fn register_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn register_st(&mut self, name: String, grade: i32) {
        let student = Student {
            name,
            grade,
            status: StudentStatus::Active,
        };

        self.students.push(student);
    }

    pub fn get_students(&self) -> Vec<Student> {
        self.students.to_vec()
    }


    pub fn edit_student(&mut self, index: usize, name: String, grade: i32) {
        if self.students.len() > index {
            self.students[index].name = name;
            self.students[index].grade = grade;
        }
    }

    pub fn update_status(&mut self, index: usize, status: StudentStatus) {
        if self.students.len() > index {
            self.students[index].status = status;
        }
    }

    pub fn delete_student(&mut self, index: usize) {
        if self.students.len() > index {
            self.students.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut management = Management::initialize();
        assert!(management.students.len() == 0);

        let student = Student {
            name: "Alice".to_string(),
            grade: 85,
            status: StudentStatus::Active,
        };

        management.register_student(student);
        assert!(management.students.len() == 1);
    }

    #[test]
    fn test_get_students(){
        let mut management = Management::initialize();
        assert!(management.students.len() == 0);

        let name_one = "Charlie".to_string();
        let grade_one = 78;

        let name_two = "Diana".to_string();
        let grade_two = 92;

        management.register_st(name_one, grade_one);
        management.register_st(name_two, grade_two);

        let students = management.get_students();

        assert_eq!(students.len(), 2);
    }
}
