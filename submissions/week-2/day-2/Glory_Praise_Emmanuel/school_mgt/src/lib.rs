#[derive(Debug, Clone)]
pub enum Status {
    True,
    False
}

#[derive(Debug, Clone)]
pub struct Student {
    pub name: String,
    pub grade: String,
    pub is_active: Status,
}

pub struct SchoolDB {
   pub register: Vec<Student>,
}

impl SchoolDB {
    pub fn new() -> Self {
      Self {
        register: Vec::new()
      }
    }

    pub fn add_new_student(&mut self, student: Student) {
      self.register.push(student);
    }

     pub fn update_student(
        &mut self,
        index: usize,
        new_data: Student,
    ) {
        if let Some(student) = self.register.get_mut(index) {
          *student = new_data;
          println!("Updated student data found at index {}", index);

        } else {
            println!("No student found at index {}", index);
        }
    }

    pub fn delete_student(&mut self, index: usize) {
        if index < self.register.len() {
            let removed = self.register.remove(index);
            println!("\nDeleted student with name: {}", removed.name);
        } else {
            println!("Invalid index: {}", index);
        }
    }
    
    pub fn view_students(&self) -> Vec<Student> {
      self.register.to_vec()
    }

    pub fn view_student(&self, index: usize) -> &Student {
      self.register.get(index).unwrap()
    }

  }
  

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_student() {
    let mut school_register = SchoolDB::new();

    assert!(school_register.register.len() == 0);

    let student = Student {
      name: "Glory Praise".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    school_register.add_new_student(student);
    assert!(school_register.register.len() == 1);
  }

  #[test]
  fn test_update_student() {
    let mut school_register = SchoolDB::new();

    assert!(school_register.register.len() == 0);

    let student = Student {
      name: "Glory Praise".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    school_register.add_new_student(student);

    assert!(school_register.register.len() == 1);

    let student2 = Student {
      name: "Paul Peter".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    school_register.update_student(0, student2);

    let student_data = school_register.view_student(0);

    assert_eq!(student_data.name, "Paul Peter".to_string());

  }

  #[test]
  fn test_delete_student() {
    let mut school_register = SchoolDB::new();

    assert!(school_register.register.len() == 0);

    let student = Student {
      name: "Glory Praise".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    let student2 = Student {
      name: "Paul Peter".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    school_register.add_new_student(student);
    school_register.add_new_student(student2);

    assert!(school_register.register.len() == 2);

    school_register.delete_student(0);

    assert!(school_register.register.len() == 1);
    
  }

  #[test]
  fn test_get_students() {
    let mut school_register = SchoolDB::new();

    assert!(school_register.register.len() == 0);

     let student = Student {
      name: "Glory Praise".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    let student2 = Student {
      name: "Paul Peter".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    school_register.add_new_student(student);
    school_register.add_new_student(student2);

    let students = school_register.view_students();

    assert_eq!(students.len(), 2);

  }

  #[test]
  fn test_get_student() {
    let mut school_register = SchoolDB::new();

    assert!(school_register.register.len() == 0);

    let student = Student {
      name: "Glory Praise".to_string(),
      grade: "Grade A".to_string(),
      is_active: Status::True,
    };

    school_register.add_new_student(student);

    let student = school_register.view_student(0);

    assert_eq!(student.name, "Glory Praise".to_string());
    assert_eq!(student.grade, "Grade A".to_string());

  }

}