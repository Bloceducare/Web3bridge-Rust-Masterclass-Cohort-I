#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub class: String,
    pub student_status: Status,
}

pub struct School {
    pub students: Vec<Student>,
    pub next_id: u32,
}

impl School {
    pub fn new() -> School {
        School {
            students: Vec::new(), next_id: 1
        }
    }

    pub fn create_student(&mut self, name: String, age: u8, class: String, student_status: Status) -> u32 {
        let id = self.next_id;
        let student = Student {
            id,
            name,
            age,
            class,
            student_status,
        };
        self.students.push(student);
        self.next_id += 1;
        id
    }

    pub fn edit_student(&mut self, id: u32, name: Option<String>, age: Option<u8>, class: Option<String>, student_status: Option<Status>) -> Result<(), String>{
        if let Some(student) = self.students.iter_mut().find(|student| student.id == id) {
            if let Some(n) = name {
                student.name = n;
            }
            if let Some(a) = age {
                student.age = a;
            }
            if let Some(c) = class {
                student.class = c;
            }
            if let Some(s) = student_status {
                student.student_status = s;
            }
            Ok(())
        } else {
            println!("Student with id {} not found", id);
            Err("Student with id {} not found".to_string())
        }
    }

    pub fn update_student_status(&mut self, id: u32, student_status: Option<Status>) -> Result<(), String>{
        if let Some(student) = self.students.iter_mut().find(|student| student.id == id) {
            if let Some(s) = student_status {
                student.student_status = s;
            }
            Ok(())
        } else {
            println!("Student with id {} not found", id);
            Err("Student with id {} not found".to_string())
        }
    }

    pub fn delete_student(&mut self, id: u32) {
        if let Some(index) = self.students.iter().position(|student| student.id == id) {
            self.students.remove(index);
            println!("Student with id {} deleted", id);
        } else {
            println!("Student with id {} not found", id);
        }
    }

    pub fn get_students(&self) -> Vec<Student> {
        self.students.to_vec()
    }

    pub fn get_student(&self, id: u32) -> &Student {
        self.students.iter().find(|student| student.id == id).unwrap()
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_student() {
        let mut school = School::new();

        assert!(school.students.len() == 0);
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);

        assert!(school.students.len() == 1);
    }

    #[test]
    fn test_get_students() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);
        school.create_student("Olaide Adenigba".to_string(), 12, "SS1".to_string(), Status::Active);

        let students = school.get_students();
        assert!(students.len() == 2);
    }

    #[test]
    fn test_delete_student() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);
        school.create_student("Olaide Adenigba".to_string(), 12, "SS1".to_string(), Status::Active);

        school.delete_student(1);
        let students = school.get_students();
        assert!(students.len() == 1);
    }

    #[test]
    fn get_student() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);

        let student = school.get_student(1);
        assert!(student.name == "Olumide Adenigba");
        assert!(student.age == 14);
        assert!(student.class == "SS3");
        assert!(student.student_status == Status::Active);
    }

    #[test]
    fn test_edit_student() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);

        school.edit_student(1, Some("Olaide Adenigba".to_string()), Some(12), Some("SS1".to_string()), Some(Status::Active));

        let student = school.get_student(1);
        assert!(student.name == "Olaide Adenigba");
        assert!(student.age == 12);
        assert!(student.class == "SS1");
        assert!(student.student_status == Status::Active);
    }

    #[test]
    fn test_update_student_status() {
        let mut school = School::new();
        school.create_student("Olumide Adenigba".to_string(), 14, "SS3".to_string(), Status::Active);

        school.update_student_status(1, Some(Status::Inactive));

        let student = school.get_student(1);
        assert!(student.student_status == Status::Inactive);
    }
    
}