// Student’s current enrolment status
#[derive(Debug, Clone, PartialEq)]
pub enum Student_Status {
    Active,
    Inactive,
}

// record for each student
#[derive(Debug, Clone, PartialEq)]
pub struct Student_Details {
    pub id: u32,
    pub name: String,
    pub grade: u8,
    pub status: Student_Status,
}

// Manager 
pub struct Class_Manager {
    next_id: u32,
    students: Vec<Student_Details>,
}

impl Class_Manager {
    
    pub fn new() -> Self {
        Self {
            next_id: 1,
            students: Vec::new(),
        }
    }

    pub fn register(&mut self, name: String, grade: u8) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        self.students.push(Student_Details {
            id,
            name,
            grade,
            status: Student_Status::Active,
        });
        id
    }

    pub fn edit(&mut self, id: u32, new_name: Option<String>, new_grade: Option<u8>) {
        if let Some(stu) = self.students.iter_mut().find(|s| s.id == id) {
            if let Some(n) = new_name {
                stu.name = n;
            }
            if let Some(g) = new_grade {
                stu.grade = g;
            }
        }
    }

    
    pub fn update_status(&mut self, id: u32, new_status: Student_Status) {
        if let Some(stu) = self.students.iter_mut().find(|s| s.id == id) {
            stu.status = new_status;
        }
    }

    pub fn delete(&mut self, id: u32) {
        self.students.retain(|s| s.id != id);
    }

    pub fn view(&self) -> &[Student_Details] {
        &self.students
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut cm = Class_Manager::new();
        let id = cm.register(String::from("Alice"), 5);

        assert_eq!(cm.view().len(), 1);
        assert_eq!(cm.view()[0].id, id);
        assert_eq!(cm.view()[0].status, Student_Status::Active);
    }

    #[test]
    fn test_edit() {
        let mut cm = Class_Manager::new();
        let id = cm.register(String::from("Ben"), 6);

        cm.edit(id, Some(String::from("Benjamin")), Some(7));

        let s = &cm.view()[0];
        assert_eq!(s.name, "Benjamin");
        assert_eq!(s.grade, 7);
    }

    #[test]
    fn test_update_status() {
        let mut cm = Class_Manager::new();
        let id = cm.register(String::from("Carla"), 8);

        cm.update_status(id, Student_Status::Inactive);

        assert_eq!(cm.view()[0].status, Student_Status::Inactive);
    }

    #[test]
    fn test_delete() {
        let mut cm = Class_Manager::new();
        let id1 = cm.register(String::from("Dan"), 4);
        let _id2 = cm.register(String::from("Eva"), 5);

        cm.delete(id1);

        assert_eq!(cm.view().len(), 1);
        assert_eq!(cm.view()[0].name, "Eva");
    }
}
