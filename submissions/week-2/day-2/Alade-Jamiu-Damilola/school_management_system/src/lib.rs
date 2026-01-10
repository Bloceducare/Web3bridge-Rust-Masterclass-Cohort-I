#[derive(Debug, Clone)]
pub enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
pub struct Student {
    id: u32,
    name: String,
    grade: u8,
    status: Status,
}

pub struct Class {
    pub students: Vec<Student>,
}

impl Class {
    pub fn initialize() -> Self {
        Class {
            students: Vec::new(),
        }
    }

    pub fn register_student(&mut self, id: u32, name: String, grade: u8, status: Status) -> usize {
        let student = Student { id, name, grade, status };
        self.students.push(student);
        self.students.len() - 1
    }

    pub fn edit_student(&mut self, index: usize, name: String, grade: u8, status: Status) -> bool {
        if let Some(student) = self.students.get_mut(index) {
            student.name = name;
            student.grade = grade;
            student.status = status;
            true
        } else {
            false
        }
    }

    pub fn update_student(
        &mut self,
        index: usize,
        _id: u32,
        name: Option<String>,
        grade: Option<u8>,
        status: Option<Status>,
    ) -> bool {
        if let Some(student) = self.students.get_mut(index) {
            if let Some(n) = name { student.name = n; }
            if let Some(g) = grade { student.grade = g; }
            if let Some(s) = status { student.status = s; }
            true
        } else {
            false
        }
    }

    pub fn delete_student(&mut self, index: usize) -> bool {
        if index < self.students.len() {
            self.students.remove(index);
            true
        } else {
            false
        }
    }

    pub fn view_student(&self, index: usize) -> &Student {
        &self.students[index]
    }

    pub fn view_all(&self) -> &[Student] {
        &self.students
    }

    pub fn find_student_by_id(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|student| student.id == id)
    }

    pub fn mark_student_active(&mut self, id: u32) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.status = Status::Active;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let class = Class::initialize();
        assert!(class.students.is_empty());
    }

    #[test]
    fn test_register_student() {
        let mut class = Class::initialize();
        let idx = class.register_student(1, "Alice".to_string(), 95, Status::Active);
        assert_eq!(idx, 0);
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].name, "Alice");
    }

    #[test]
    fn test_edit_student() {
        let mut class = Class::initialize();
        class.register_student(1, "Bob".to_string(), 80, Status::Inactive);
        let success = class.edit_student(0, "Robert".to_string(), 85, Status::Active);
        assert!(success);
        assert_eq!(class.students[0].name, "Robert");
        assert_eq!(class.students[0].grade, 85);
        assert!(matches!(class.students[0].status, Status::Active));
        let failure = class.edit_student(99, "NoOne".to_string(), 0, Status::Inactive);
        assert!(!failure);
    }

    #[test]
    fn test_update_student() {
        let mut class = Class::initialize();
        class.register_student(2, "Charlie".to_string(), 75, Status::Active);
        let success = class.update_student(0, 2, None, Some(90), None);
        assert!(success);
        assert_eq!(class.students[0].grade, 90);
        let success = class.update_student(0, 2, Some("Charles".to_string()), None, Some(Status::Inactive));
        assert!(success);
        assert_eq!(class.students[0].name, "Charles");
        assert!(matches!(class.students[0].status, Status::Inactive));
        let failure = class.update_student(99, 2, None, None, None);
        assert!(!failure);
    }

    #[test]
    fn test_delete_student() {
        let mut class = Class::initialize();
        class.register_student(3, "Dave".to_string(), 88, Status::Active);
        class.register_student(4, "Eve".to_string(), 92, Status::Active);
        assert_eq!(class.students.len(), 2);
        let success = class.delete_student(0);
        assert!(success);
        assert_eq!(class.students.len(), 1);
        assert_eq!(class.students[0].name, "Eve");
        let failure = class.delete_student(5);
        assert!(!failure);
    }

    #[test]
    fn test_view_student() {
        let mut class = Class::initialize();
        class.register_student(5, "Frank".to_string(), 70, Status::Inactive);
        let student = class.view_student(0);
        assert_eq!(student.name, "Frank");
    }

    #[test]
    fn test_view_all() {
        let mut class = Class::initialize();
        class.register_student(6, "Grace".to_string(), 85, Status::Active);
        class.register_student(7, "Heidi".to_string(), 90, Status::Active);
        let all = class.view_all();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].name, "Grace");
        assert_eq!(all[1].name, "Heidi");
    }

    #[test]
    fn test_find_student_by_id() {
        let mut class = Class::initialize();
        class.register_student(10, "Ivan".to_string(), 95, Status::Active);
        class.register_student(11, "Judy".to_string(), 88, Status::Inactive);
        let found = class.find_student_by_id(10);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Ivan");
        let not_found = class.find_student_by_id(99);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_mark_student_active() {
        let mut class = Class::initialize();
        class.register_student(12, "Mallory".to_string(), 75, Status::Inactive);
        let success = class.mark_student_active(12);
        assert!(success);
        assert!(matches!(class.students[0].status, Status::Active));
        let failure = class.mark_student_active(99);
        assert!(!failure);
    }

    #[test]
    fn test_integration() {
        let mut class = Class::initialize();
        class.register_student(1, "Alice".to_string(), 90, Status::Active);
        class.register_student(2, "Bob".to_string(), 80, Status::Inactive);
        class.register_student(3, "Charlie".to_string(), 85, Status::Active);
        assert_eq!(class.students.len(), 3);
        class.mark_student_active(2);
        let bob = class.find_student_by_id(2).unwrap();
        assert!(matches!(bob.status, Status::Active));
        class.edit_student(2, "Charles".to_string(), 95, Status::Active);
        assert_eq!(class.students[2].grade, 95);
        class.delete_student(0);
        assert_eq!(class.students.len(), 2);
        assert_eq!(class.students[0].name, "Bob");
    }
}