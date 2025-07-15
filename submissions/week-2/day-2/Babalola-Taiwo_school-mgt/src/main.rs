fn main() {
    let mut my_class = ClassMgt::new_class();

    my_class.register_student("Babalola Taiwo".to_string(), "Grade 10".to_string());
    my_class.register_student("Ada Okeke".to_string(), "Grade 12".to_string());

    my_class.view_all_students();

    my_class.edit_student(0, "Taiwo Babalola".to_string(), "Grade 11".to_string());

    my_class.update_student_status(1, StudentStatus::Inactive);

    my_class.view_all_students();

    my_class.delete_student(0);

    my_class.view_all_students();
}

#[derive(Debug, Clone)]
pub enum StudentStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub grade: String,
    pub status: StudentStatus,
}

pub struct ClassMgt {
    pub student_list: Vec<Student>,
}

impl ClassMgt {
    pub fn new_class() -> ClassMgt {
        ClassMgt {
            student_list: Vec::new(),
        }
    }

    pub fn register_student(&mut self, name: String, grade: String) {
        let student = Student {
            name,
            grade,
            status: StudentStatus::Active,
        };
        self.student_list.push(student);
        println!("Student registered successfully!");
    }

    pub fn edit_student(&mut self, index: usize, new_name: String, new_grade: String) {
        if let Some(student) = self.student_list.get_mut(index) {
            student.name = new_name;
            student.grade = new_grade;
            println!("Student details updated successfully!");
        } else {
            println!("Student not found!");
        }
    }

    pub fn update_student_status(&mut self, index: usize, new_status: StudentStatus) {
        if let Some(student) = self.student_list.get_mut(index) {
            student.status = new_status;
            println!("Student status updated successfully!");
        } else {
            println!("Student not found!");
        }
    }

    pub fn delete_student(&mut self, index: usize) {
        if index < self.student_list.len() {
            self.student_list.remove(index);
            println!("Student deleted successfully!");
        } else {
            println!("Student not found!");
        }
    }

    pub fn view_all_students(&self) {
        if self.student_list.is_empty() {
            println!("No students in the class.");
        } else {
            for (index, student) in self.student_list.iter().enumerate() {
                let status = match student.status {
                    StudentStatus::Active => "Active",
                    StudentStatus::Inactive => "Inactive",
                };
                println!(
                    "{}. {} | Grade: {} | Status: {}",
                    index + 1,
                    student.name,
                    student.grade,
                    status
                );
            }
        }
    }
}


// TEST FIlE
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut class = ClassMgt::new_class();
        assert_eq!(class.student_list.len(), 0);

        class.register_student("John Doe".to_string(), "Grade 9".to_string());
        assert_eq!(class.student_list.len(), 1);
        assert_eq!(class.student_list[0].name, "John Doe");
        assert_eq!(class.student_list[0].grade, "Grade 9");
        assert!(matches!(class.student_list[0].status, StudentStatus::Active));
    }

    #[test]
    fn test_edit_student() {
        let mut class = ClassMgt::new_class();
        class.register_student("Jane Doe".to_string(), "Grade 8".to_string());

        class.edit_student(0, "Jane Smith".to_string(), "Grade 10".to_string());

        assert_eq!(class.student_list[0].name, "Jane Smith");
        assert_eq!(class.student_list[0].grade, "Grade 10");
    }

    #[test]
    fn test_update_student_status() {
        let mut class = ClassMgt::new_class();
        class.register_student("Mark Brown".to_string(), "Grade 7".to_string());

        class.update_student_status(0, StudentStatus::Inactive);

        assert!(matches!(class.student_list[0].status, StudentStatus::Inactive));
    }

    #[test]
    fn test_delete_student() {
        let mut class = ClassMgt::new_class();
        class.register_student("Lucy Green".to_string(), "Grade 11".to_string());

        assert_eq!(class.student_list.len(), 1);

        class.delete_student(0);

        assert_eq!(class.student_list.len(), 0);
    }
}