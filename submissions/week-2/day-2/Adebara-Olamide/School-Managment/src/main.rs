struct Student {
    name: String,
    grade: u8,
    status: Status,
}

enum Status {
    Active,
    InActive,
}

struct School {
    students: Vec<Student>,
}

impl School {
    fn new() -> School {
        School {
            students: Vec::new(),
        }
    }

    fn add_student(&mut self, name: &str, grade: u8, status: Status) {
        let student = Student {
            name: String::from(name),
            grade,
            status,
        };
        self.students.push(student);
    }

    fn edit_student(&mut self, name: &str, new_name: &str, new_grade: u8, new_status: Status) {
        if let Some(student) = self.students.iter_mut().find(|s| s.name == name) {
            student.name = new_name.to_string();
            student.grade = new_grade;
            student.status = new_status;
            println!("Student edited!");
        } else {
            println!("Student not found!");
        }
    }

    fn remove_student(&mut self, name: &str) {
        self.students.retain(|student| student.name != name);
    }

    fn view_students(&self, index: usize) -> &Student {
        self.students.get(index).expect("Student not found")
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_student() {
        let mut school = School::new();
        school.add_student("Adebara Olamide", 10, Status::Active);
        assert_eq!(school.students.len(), 1);
        assert_eq!(school.students[0].name, "Adebara Olamide");
    }

    #[test]
    fn test_edit_student() {
        let mut school = School::new();
        school.add_student("Adebara Olamide", 10, Status::Active);
        school.edit_student("Adebara Olamide", "Adebara Holamite", 75, Status::Active);
        assert_eq!(school.students[0].name, "Adebara Holamite");
        assert_eq!(school.students[0].grade, 75);
    }

    #[test]
    fn test_remove_student() {
        let mut school = School::new();
        school.add_student("Adebara Olamide", 10, Status::Active);
        assert_eq!(school.students.len(), 1);
        school.remove_student("Adebara Olamide");
        assert_eq!(school.students.len(), 0);
    }

    #[test]
    fn test_view_students() {
        let mut school = School::new();
        school.add_student("Adebara Olamide", 10, Status::Active);
        school.view_students(1);
        assert_eq!(school.students[0].name, "Adebara Olamide");
        assert_eq!(school.students[0].grade, 10);
    }
}
