#[derive(Debug)]
pub struct Student {
    name: String,
    active: IsActive,
}

#[derive(Debug)]
pub struct StudentList {
    students: Vec<Student>,
}

#[derive(Debug)]
pub enum IsActive {
    Active,
    Inactive,
}

impl Student {
    fn new(name: String) -> Self {
        Student {
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

    pub fn add_active_student(&mut self, id: usize) {
        match self.students.get_mut(id - 1) {
            Some(student) => student.active = IsActive::Active,

            None => panic!("Nothing is here"),
        }
    }

    pub fn add(&mut self, name: String) {
        self.students.push(Student::new(name));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_students() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string());
        new_students.add("Xcel".to_string());

        assert!(new_students.students.len() > 0);
    }

    #[test]
    fn test_get_student() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string());
        new_students.add("Xcel".to_string());

        let student = new_students.get_student(1).unwrap();
        assert_eq!(student.name, "oluwagbemiga");
    }

    #[test]
    fn test_edit_student() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string());
        new_students.add("Xcel".to_string());

        new_students.edit(1, "bestie".to_string(), IsActive::Active);
        assert_eq!(new_students.get_student(2).unwrap().name, "bestie");
    }

    #[test]
    fn test_delete() {
        let mut new_students = StudentList::new();
        new_students.add("oluwagbemiga".to_string());
        new_students.add("Xcel".to_string());
        new_students.add("Bestiee".to_string());

        new_students.delete(1);

        for student in new_students.students {
            assert!(student.name != "Xcel")
        }
    }
}
