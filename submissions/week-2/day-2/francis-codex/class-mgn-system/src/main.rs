#[derive(Clone, Debug, PartialEq)]

pub enum StudentStatus {
    Active,
    Inactive,
}

pub struct Student {
    pub name: String,
    pub age: u8, 
    pub id: u32,
    pub status: StudentStatus,
}

pub struct ClassManager {
    pub students: Vec<Student>,
}

impl ClassManager {
    pub fn new() -> Self {
        Self {
            students: Vec::new(),
        }
    }

    pub fn register_student(&mut self, name: String, age: u8, id: u32) {
        let student = Student {
            name,
            age,
            id,
            status: StudentStatus::Active,
        };
        self.students.push(student);
    }

    pub fn edit_student(&mut self, name: &str, new_name: Option<String>, new_age: Option<u8>, new_id: Option<u32>) {
        for student in &mut self.students {
            if student.name == name {
                if let Some(ref n) = new_name {
                    student.name = n.to_string();
                }
                if let Some(a) = new_age {
                    student.age = a;
                }
                 if let Some(a) = new_id {
                    student.id = a;
                }
            }
        }
    }

    pub fn update_student_status(&mut self, name: &str, status: StudentStatus) {
        for student in &mut self.students {
            if student.name == name {
                student.status = status.clone();
            }
        }
    }

    pub fn delete_student(&mut self, name: &str) {
        self.students.retain(|student| student.name != name);
    }

    pub fn list_students(&self) {
        for student in &self.students {
            println!("Name: {}, Age: {}, ID: {}, Status: {:?}", student.name, student.age, student.id, student.status);
        }
    }
}

fn main() {
    let mut manager = ClassManager::new();
    manager.register_student("francis ".to_string(), 20, 1);
    manager.register_student("codex".to_string(), 22, 2);
    manager.register_student("john".to_string(), 30, 3);
    manager.list_students();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut manager = ClassManager::new();
        manager.register_student("francis codex".to_string(), 85, 1);
        assert_eq!(manager.students.len(), 1);
        assert_eq!(manager.students[0].name, "francis codex");
        assert_eq!(manager.students[0].age, 85);
        assert_eq!(manager.students[0].status, StudentStatus::Active);
    }

    #[test]
    fn test_edit_student() {
        let mut manager = ClassManager::new();
        manager.register_student("francis ".to_string(), 20, 1);
        manager.register_student("codex".to_string(), 22, 2);
        
        manager.edit_student("francis ", Some("francis codex".to_string()), Some(21), Some(1));
        assert_eq!(manager.students[0].name, "francis codex");
        assert_eq!(manager.students[0].age, 21);
        
        manager.edit_student("codex", Some("Joshua".to_string()), None, Some(1));
        assert_eq!(manager.students[1].name, "Joshua");
        assert_eq!(manager.students[1].age, 22);
        
        manager.edit_student("francis codex", None, Some(23), Some(1));
        assert_eq!(manager.students[0].name, "francis codex");
        assert_eq!(manager.students[0].age, 23);
    }

    #[test]
    fn test_update_student_status() {
        let mut manager = ClassManager::new();
        manager.register_student("francis ".to_string(), 20, 1);
        manager.register_student("codex".to_string(), 22, 2);
        
        assert_eq!(manager.students[0].status, StudentStatus::Active);
        assert_eq!(manager.students[1].status, StudentStatus::Active);
        
        manager.update_student_status("francis ", StudentStatus::Inactive);
        assert_eq!(manager.students[0].status, StudentStatus::Inactive);
        assert_eq!(manager.students[1].status, StudentStatus::Active);
        
        manager.update_student_status("codex", StudentStatus::Inactive);
        assert_eq!(manager.students[0].status, StudentStatus::Inactive);
        assert_eq!(manager.students[1].status, StudentStatus::Inactive);
    }

    #[test]
    fn test_delete_student() {
        let mut manager = ClassManager::new();
        manager.register_student("francis ".to_string(), 20, 1);
        manager.register_student("codex".to_string(), 22, 2);
        manager.register_student("John".to_string(), 19, 3);
        
        assert_eq!(manager.students.len(), 3);
        
        manager.delete_student("codex");
        assert_eq!(manager.students.len(), 2);
        assert_eq!(manager.students[0].name, "francis ");
        assert_eq!(manager.students[1].name, "John");
        
        manager.delete_student("francis ");
        assert_eq!(manager.students.len(), 1);
        assert_eq!(manager.students[0].name, "John");
        
        manager.delete_student("John");
        assert_eq!(manager.students.len(), 0);
    }

    #[test]
    fn test_list_students() {
        let mut manager = ClassManager::new();
        manager.register_student("francis ".to_string(), 20, 1);
        manager.register_student("codex".to_string(), 22, 2);
        manager.update_student_status("francis ", StudentStatus::Inactive);
        
        manager.list_students();
    }
}

