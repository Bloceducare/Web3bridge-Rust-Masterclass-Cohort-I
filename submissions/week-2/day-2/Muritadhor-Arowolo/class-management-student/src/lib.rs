#[derive(Debug)]
#[derive(PartialEq)]
pub enum Grade{
    GradeA,
    GradeB,
    GradeC,
    GradeD,
    GradeE,
    GradeF,
}

#[derive(PartialEq)]
#[derive(Debug, Copy, Clone)]
pub enum Status{
    Active,
    Inactive,
}

#[derive(Debug, PartialEq)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub grade: Grade,
    pub status: Status,
}

pub struct Class {
    pub class_name: String,
    pub students: Vec<Student>,
}

impl Class {
    pub fn new(class_name: String) -> Self {
        Class {
            class_name,
            students: Vec::new(),
        }
    }

    pub fn add_student(&mut self, id: u32, name: String, age: u8, grade: Grade, status: Status) {
        let student = Student {
            id,
            name,
            age,
            grade,
            status,
        };
        self.students.push(student);
    }

    pub fn get_students(&self) -> &Vec<Student> {
        &self.students
    }

    pub fn get_student(&self, index: usize) -> &Student {
        self.students.get(index).unwrap()
    }

    pub fn remove_student(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        }
    }

    pub fn update_student_status(&mut self, id: u32, status: Status){
        self.students.iter_mut().find(|student| student.id == id).unwrap().status = status;
    }

    pub fn update_student(&mut self, index: usize, name: String, age: u8, grade: Grade, status: Status) {
        if let Some(student) = self.students.get_mut(index) {
            student.name = name;
            student.age = age;
            student.grade = grade;
            student.status = status;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_student() {
        let mut class = Class::new(String::from("Math 101"));
        class.add_student(1,String::from("Joshua"), 20, Grade::GradeC, Status::Active);
        class.add_student(1, String::from("Satoshi"), 22, Grade::GradeB, Status::Inactive);
        assert_eq!(class.get_students().len(), 2);
    }

    #[test]
    fn test_remove_student() {
        let mut class = Class::new(String::from("Math 101"));
        class.add_student(1,String::from("Joshua"), 20, Grade::GradeC, Status::Active);
        assert_eq!(class.get_students().len(), 1);
        class.remove_student(0);
        assert_eq!(class.get_students().len(), 0);
    }
    
    #[test]
    fn test_get_student() {
        let mut class = Class::new(String::from("Math 101"));
        class.add_student(1, String::from("Joshua"), 20, Grade::GradeC, Status::Active);
        let new_student = Student{id: 1, name: String::from("Joshua"), age: 20, grade: Grade::GradeC, status: Status::Active};
        let student = class.get_student(0);
        // assert_eq!(student.name, "Joshua");
        // assert_eq!(student.age, 20);
        // assert_eq!(student.grade, Grade::GradeC);
        // assert_eq!(student.status, Status::Active);
        assert_eq!(*student, new_student)
    }

    #[test]
    fn test_update_student() {
        let mut class = Class::new(String::from("Math 101"));
        class.add_student(1, String::from("Joshua"), 20, Grade::GradeC, Status::Active);
        assert_eq!(class.get_students().len(), 1);
        let student = class.get_student(0);
        assert_eq!(student.name, "Joshua");
        assert_eq!(student.age, 20);
        assert_eq!(student.grade, Grade::GradeC);
        assert_eq!(student.status, Status::Active);
        class.update_student(0, String::from("John"), 21, Grade::GradeB, Status::Inactive);
        let student = class.get_student(0);
        assert_eq!(student.name, "John");
        assert_eq!(student.age, 21);
        assert_eq!(student.grade, Grade::GradeB);
        assert_eq!(student.status, Status::Inactive);
    }

    #[test]
    fn test_update_status() {
        let mut class = Class::new(String::from("Math 101"));
        class.add_student(1, String::from("Joshua"), 20, Grade::GradeC, Status::Active);
        assert_eq!(class.get_students().len(), 1);
        let student = class.get_student(0);
        assert_eq!(student.status, Status::Active);
        class.update_student_status(1, Status::Inactive);
        let student = class.get_student(0);
        assert_eq!(student.status, Status::Inactive);
    }
}