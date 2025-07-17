

use std::fmt::Display;

#[derive(Debug)]
struct Student{
    id: u32,
    name: String,
    grade: u8,
    status: Status
}

#[derive(Debug, Clone, PartialEq)]
enum Status{
    Active,
    Inactive,
}

pub struct School{
    pub students: Vec<Student>
}

impl School{
    pub fn new() -> Self{
        School{
            students: Vec::new()
        }
    }

    pub fn register_student(&mut self, student: Student){
        self.students.push(student);
    }

    pub fn edit_student(&mut self, name: String, grade: u8){
        for student in &mut self.students{
            if student.name == name{
                student.grade = grade;
            }
        }
    }

    pub fn update_status(&mut self, id: u32, status: Status){
      if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.status = status;
        } else {
            println!("Student not found");
        }
    }

    pub fn delete_student(&mut self, id: u32){
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.status = Status::Inactive;
        } else {
            println!("Student not found");
        }
    }

   
    pub fn get_student_by_id_and_set_status(&mut self, id: u32, status: Status){

        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            match student.status {
                Status::Active => {
                    student.status = status.clone();
                },
                Status::Inactive => {
                    println!("Student is already inactive");
                }

            }
        } else {
            println!("Student not found");
        }
    }
    pub fn view_students(&self){
        for student in &self.students{
            println!("Name: {:?}, Grade: {:?}, Status: {:?}", student.name, student.grade, student.status);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register_student(){
        let mut school = School::new();
        let student = Student{
            id: 1,
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        assert_eq!(school.students.len(), 1);
    }

    #[test]
    fn test_edit_student(){
        let mut school = School::new();
        let student = Student{
            id: 1,
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        school.edit_student(String::from("Ekezie Uchenna"), 11);
        assert_eq!(school.students[0].grade, 11);
    }

     #[test]

     fn test_update_status(){
        let mut school = School::new();
        let student = Student{
            id: 1,
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        school.update_status(1, Status::Inactive);
        assert_eq!(school.students[0].status, Status::Inactive);
     }

     #[test]

     fn test_delete_student(){
        let mut school = School::new();
        let student = Student{
            id: 1,
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        school.delete_student(1);
        assert_eq!(school.students[0].status, Status::Inactive);
     }
#[test]
     fn test_view_student(){
        let mut school = School::new();
        let student1 = Student{
            id: 1,
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        let student2 = Student{
            id: 2,
            name: String::from("Josh Tutor"),
            grade: 20,
            status: Status::Inactive
        };
        school.register_student(student1);
        school.register_student(student2);

        school.view_students();
     }

    #[test]
    fn test_get_student_by_id_and_set_status(){
        let mut school = School::new();
        let student = Student{
            id: 1,
            name: String::from("Ekezie Uchenna"),
            grade: 10,
            status: Status::Active
        };
        school.register_student(student);
        school.get_student_by_id_and_set_status(1, Status::Inactive);
        assert_eq!(school.students[0].status, Status::Inactive);
    }
}

