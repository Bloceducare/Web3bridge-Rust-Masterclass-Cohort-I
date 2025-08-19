fn main() {
    let mut class = ClassOfStudents::init_class();
    class.create_student("Sogo".to_string(), 100.0, ActivesStatus::NotActive);
    class.create_student("Shogzymamadowski".to_string(), 100.0, ActivesStatus::Active);
    class.update_active_status(0);
    class.update_active_status(1);
}

#[derive(Debug, Clone)]
enum ActivesStatus {
    Active,
    NotActive,
}

#[derive(Debug, Clone)]
struct Student {
    id: usize,
    name: String,
    grade: f64,
    status: ActivesStatus,
}

#[derive(Debug, Clone)]
struct ClassOfStudents {
    class_students: Vec<Student>,
}

impl ClassOfStudents {
    fn init_class() -> ClassOfStudents {
        ClassOfStudents {
            class_students: Vec::new(),
        }
    }

    fn create_student(&mut self, name: String, grade: f64,  status: ActivesStatus) {
        let new_student = Student {
            id: self.class_students.len(),
            name,
            grade,
            status
        };
        self.class_students.push(new_student);
    }

    fn get_all_students(&self) -> Vec<Student> {
        self.class_students.to_vec()
    }

    fn get_each_student(&self, id: usize) -> Student {
        if self.class_students.len() > id {
            self.class_students[id].clone()
        } else {
            panic!("student does not exist");
        }
    }

    fn update_student(&mut self, id: usize, student: Student) {
        if self.class_students.len() > id {
            self.class_students[id] = student;
        } else {
            panic!("student does not exist");
        }
    }

    fn update_active_status(&mut self, id: usize) -> Student {
        if self.class_students.len() > id {
            let student_to_update = self.class_students[id].clone();
            let updated_student = Student {
                status: match student_to_update.status {
                    ActivesStatus::Active => ActivesStatus::NotActive,
                    ActivesStatus::NotActive => ActivesStatus::Active,
                },
                ..student_to_update
            };
            println!("Updated student is {:?}", &updated_student);
            updated_student
        } else {
            panic!("student does not exist");
        }
    }

    fn delete_student(&mut self, id: usize) {
        if self.class_students.len() > id {
            self.class_students.remove(id);
        } else {
            panic!("student does not exist");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_student() {
        let mut class = ClassOfStudents::init_class();
        assert!(class.class_students.len() == 0);
        class.create_student("Sogo".to_string(), 100.0, ActivesStatus::NotActive);
        assert!(class.class_students.len() == 1);
    }

    #[test]
    fn test_get_all_students() {}
}
