#![allow(unused)]

#[derive(Clone, Debug, PartialEq)]

pub enum StudentStatus {
  ACTIVE,
  INACTIVE,
}

#[derive(Debug)]
pub struct Student {
  pub id: u32,
  pub name: String,
  pub grade: u32,
  pub gender: String,
  pub status: StudentStatus,
}


pub struct School {
  pub students: Vec<Student>,
}

impl School {
  pub fn new() -> Self {
    Self { students: Vec::new() }
  }

  pub fn register_student(&mut self, id: u32, name: String, grade: u32, gender: String) {
    let student = Student { id, name, grade, gender, status: StudentStatus::ACTIVE };
    self.students.push(student);
  }

  pub fn get_student(&self, index: usize) -> &Student {
    self.students.get(index).unwrap()
  }

  pub fn get_student_by_name(&self, name: String) -> Option<&Student> {
    self.students.iter().find(|s| s.name == name)
  }

  pub fn get_all_students(&self) -> &Vec<Student> {
    &self.students
  }
  pub fn update_student(&mut self, index: usize, name: String, grade: u32, gender: String) {
    let student = self.students.get_mut(index).unwrap();
    student.name = name;
    student.grade = grade;
    student.gender = gender;
    student.status = StudentStatus::ACTIVE;
  }

  pub fn delete_student(&mut self, index: usize) {
    self.students.remove(index);
  }

  pub fn set_student_status(&mut self, id: u32, status: StudentStatus) -> bool {
    if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
      println!("Student found: {:?}", student);
      student.status = status;
      true
    } else {
      false
    }
  }

  pub fn get_student_by_id(&self, id: u32) -> Option<&Student> {
    self.students.iter().find(|s| s.id == id)
  }
  
}


