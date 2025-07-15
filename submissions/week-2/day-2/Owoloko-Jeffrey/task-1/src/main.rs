#[derive(Debug)]
enum Active{
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
pub struct Class{
    pub students: Vec<Student>,
}

impl Student{
    fn new(id: u32, name: String, grade: i32) -> Self {
        Self { id, name, grade, active: Active::Inactive }
    }

    fn update_student(&mut self, id: u32, name: String, grade: i32) {
        self.id = id;
        self.name = name;
        self.grade = grade;
    }

    fn mark_student_active(&mut self) {
        self.active = Active::Active;
    }

    fn mark_student_inactive(&mut self) {
        self.active = Active::Inactive;
    }
}

impl Class{
    fn new() -> Self {
        Self { students: Vec::new() }
    }

    fn register_student(&mut self, student: Student) {
        self.students.push(student);
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
    let mut student = Student::new(1, "badman dev".to_string(), 3);
    class.register_student(student);
    class.view_students();
    class.delete_student(1);
    println!("Class: {:?}", class);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_student() {
        let student = Student::new(1, "John Doe".to_string(), 85);
        assert_eq!(student.id, 1);
        assert_eq!(student.name, "John Doe");
        assert_eq!(student.grade, 85);
    }

    #[test]
    fn test_create_class() {
        let class = Class::new();
        assert!(class.students.len() == 0);
    }

    #[test]
    fn test_register_student(){
        let mut class = Class::new();
        assert!(class.students.len() == 0);

        let student = Student::new(1, "Alice Smith".to_string(), 92);
        class.register_student(student);
        assert!(class.students.len() == 1);
    }

    #[test]
    fn test_delete_student(){
        let mut class = Class::new();
        
        let student1 = Student::new(1, "Bob Johnson".to_string(), 78);
        let student2 = Student::new(2, "Carol Brown".to_string(), 88);
        
        class.register_student(student1);
        class.register_student(student2);
        assert_eq!(class.students.len(), 2);

        class.delete_student(1);
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].id, 2);
    }

    #[test]
    fn test_update_student(){
        let mut student = Student::new(1, "David Wilson".to_string(), 75);
        
        student.update_student(1, "David W. Wilson".to_string(), 80);
        
        assert_eq!(student.name, "David W. Wilson");
        assert_eq!(student.grade, 80);
    }

    #[test]
    fn test_mark_student_active(){
        let mut student = Student::new(1, "Emma Davis".to_string(), 90);
        
        student.mark_student_active();
        
        match student.active {
            Active::Active => assert!(true),
            Active::Inactive => assert!(false, "Student should be active"),
        }
    }

    #[test]
    fn test_mark_student_inactive(){
        let mut student = Student::new(1, "Frank Miller".to_string(), 82);
        
        student.mark_student_active();
        student.mark_student_inactive();
        
        match student.active {
            Active::Inactive => assert!(true),
            Active::Active => assert!(false, "Student should be inactive"),
        }
    }

}