#[derive(Debug)]
enum Active {
    Active,
    Inactive,
}

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    grade: i32,
    active: Active,
}

#[derive(Debug)]
pub struct Class {
    pub students: Vec<Student>,
    next_id: u32,
}

impl Class {
    fn new() -> Self {
        Self {
            students: Vec::new(),
            next_id: 0,
        }
    }

    fn register_student(&mut self, name: String, grade: i32) {
        self.next_id += 1;
        let student_id = self.next_id;

        let student = Student {
            id: student_id,
            name,
            grade,
            active: Active::Inactive,
        };

        self.students.push(student);
    }

    fn update_student(&mut self, id: u32, name: String, grade: i32) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.name = name;
            student.grade = grade;
        }
    }

    fn mark_student_active(&mut self, id: u32) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.active = Active::Active;
        }
    }

    fn mark_student_inactive(&mut self, id: u32) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.active = Active::Inactive;
        }
    }

    fn delete_student(&mut self, id: u32) {
        if let Some(index) = self.students.iter().position(|x| x.id == id) {
            self.students.remove(index);
        } else {
            println!("Student not found");
        }
    }
    fn view_students(&self) {
        for student in &self.students {
            println!("Student: {:?}", student);
        }
    }
}

fn main() {
    let mut class = Class::new();
    class.register_student("badman dev".to_string(), 3);

    class.view_students();
    class.delete_student(1);
    println!("Class: {:?}", class);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_class() {
        let class = Class::new();
        assert!(class.students.len() == 0);
    }

    #[test]
    fn test_register_student() {
        let mut class = Class::new();
        class.register_student("Alice Smith".to_string(), 92);
        let student = &class.students[0];
        assert_eq!(student.id, 1);
        assert_eq!(student.name, "Alice Smith");
        assert_eq!(student.grade, 92);
        assert!(class.students.len() == 1);
    }

    #[test]
    fn test_delete_student() {
        let mut class = Class::new();

        class.register_student("Bob Johnson".to_string(), 78);
        class.register_student("Carol Brown".to_string(), 88);
        assert_eq!(class.students.len(), 2);

        class.delete_student(1);
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].id, 2);
    }

    #[test]
    fn test_update_student() {
        let mut class = Class::new();
        class.register_student("David W. Wilson".to_string(), 75);
        class.update_student(1, "David W. Wilson".to_string(), 80);

        assert_eq!(class.students[0].name, "David W. Wilson");
        assert_eq!(class.students[0].grade, 80);
    }

    #[test]
    fn test_mark_student_active() {
        let mut class = Class::new();
        class.register_student("David W. Wilson".to_string(), 75);

        class.mark_student_active(1);

        match class.students[0].active {
            Active::Active => assert!(true),
            Active::Inactive => assert!(false, "Student should be active"),
        }
    }

    #[test]
    fn test_mark_student_inactive() {
        let mut class = Class::new();
        class.register_student("David W. Wilson".to_string(), 75);

        class.mark_student_inactive(1);

        match class.students[0].active {
            Active::Inactive => assert!(true),
            Active::Active => assert!(false, "Student should be inactive"),
        }
    }
}
