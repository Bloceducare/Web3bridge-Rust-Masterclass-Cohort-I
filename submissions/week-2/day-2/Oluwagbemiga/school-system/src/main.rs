#[derive(Debug)]
pub struct Student {
    id: usize,
    name: String,
    active: IsActive,
}

#[derive(Debug)]
pub struct StudentList {
    students: Vec<Student>,
}

#[derive(Debug, PartialEq)]
pub enum IsActive {
    Active,
    Inactive,
}

impl Student {
    fn new(name: String, id: usize) -> Self {
        Student {
            id,
            name,
            active: IsActive::Inactive,
        }
    }
}

impl StudentList {
    pub fn new() -> Self {
        StudentList { students: vec![] }
    }

    pub fn get_student(&self, index: usize) -> Option<&Student> {
        self.students.get(index - 1)
    }

    pub fn all_students(&self) -> &Vec<Student> {
        &self.students
    }

    pub fn toggle_activity(&mut self, id: usize) {
        for student in &mut self.students {
            if student.id == id {
                if student.active == IsActive::Inactive {
                    student.active = IsActive::Active;
                } else {
                    student.active = IsActive::Inactive;
                }
            }
        }
    }

    // fn get_active_student(&mut self, student_id: usize) {

    //     match self.students.get(student_id) {}
    // }
    pub fn add(&mut self, name: String, id: usize) {
        self.students.push(Student::new(name, id));
    }

    pub fn remove(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        } else {
            println!("No student found at index {}", index);
        }
    }
    pub fn edit(&mut self, index: usize, new_name: String, new_active: IsActive) {
        if index < self.students.len() {
            self.students[index].name = new_name;
            self.students[index].active = new_active;
        } else {
            println!("No student found at index {}", index);
        }
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        } else {
            println!("No student found at index {}", index);
        }
    }
}

fn main() {
    let mut new_students = StudentList::new();
    new_students.add("oluwagbemiga".to_string(), 1);
    new_students.add("Xcel".to_string(), 2);
    new_students.add("Bestiee".to_string(), 3);

    new_students.toggle_activity(3);
    new_students.toggle_activity(2);
    println!(
        "After toggling activity for student at index 3: {:#?}",
        new_students.all_students()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_students() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string(), 1);
        new_students.add("Xcel".to_string(), 2);

        assert!(new_students.students.len() > 0);
    }

    #[test]
    fn test_get_student() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string(), 1);
        new_students.add("Xcel".to_string(), 2);

        let student = new_students.get_student(1).unwrap();
        assert_eq!(student.name, "oluwagbemiga");
    }

    #[test]
    fn test_edit_student() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string(), 1);
        new_students.add("Xcel".to_string(), 2);

        new_students.edit(1, "bestie".to_string(), IsActive::Active);
        assert_eq!(new_students.get_student(2).unwrap().name, "bestie");
    }

    #[test]
    fn test_delete() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string(), 1);
        new_students.add("Xcel".to_string(), 2);
        new_students.add("Bestiee".to_string(), 3);

        new_students.delete(1);

        for student in new_students.students {
            assert!(student.name != "Xcel")
        }
    }
    #[test]
    fn test_toggle_activity() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string(), 1);
        new_students.add("Xcel".to_string(), 2);
        new_students.add("Bestiee".to_string(), 3);

        new_students.toggle_activity(3);

        for student in new_students.students {
            if student.id == 3 {
                assert_eq!(student.active, IsActive::Active);
            } else {
                assert_eq!(student.active, IsActive::Inactive);
            }
        }
    }
}
