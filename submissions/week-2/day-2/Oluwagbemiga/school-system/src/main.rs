struct Student {
    id: u32,
    name: String,
    isActive: IsActive,
}

impl Student {
    fn new(name: String, id: u32) -> Self {
        Student {
            id,
            name,
            isActive: IsActive::Active,
        }
    }
}

#[derive(Debug, PartialEq)]
enum IsActive {
    Active,
    Inactive,
}

struct Students {
    data: Vec<Student>,
    student_id: u32,
}

impl Students {
    fn new() -> Self {
        Students {
            data: Vec::new(),
            student_id: 1,
        }
    }

    fn iterator(students: &mut Self, id: u32) -> &mut Student {
        students.data.iter_mut().find(|el| el.id == id).unwrap()
    }

    fn add(&mut self, name: String) {
        let new_student = Student::new(name, self.student_id);
        self.data.push(new_student);
        self.student_id += 1;
    }

    fn leave(&mut self, id: u32) {
        self.data.iter().find(|el| el.id != id).unwrap();
    }

    fn get_active_students(&self) -> Vec<&Student> {
        self.data
            .iter()
            .filter(|el| el.isActive == IsActive::Active)
            .collect::<Vec<_>>()
    }

    fn edit_details(&mut self, id: u32, new_name: String) {
        let mut detail = Students::iterator(self, id);
        detail.name = new_name;
    }

    fn deactivate(&mut self, id: u32) {
        let mut detail = Students::iterator(self, id);
        detail.isActive = IsActive::Inactive;
    }
}

fn main() {
    print!("Here we go!!!!!!")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_students() -> Students {
        let mut students = Students::new();
        students.add("Bestiee".to_string());
        students.add("Oluwagbemiga".to_string());
        students
    }

    #[test]
    fn test_add_student() {
        let mut students = create_students();
        students.add("Xcel".to_string());
        assert_eq!(students.data.len(), 3);
        assert_eq!(students.data[2].name, "Xcel");
    }

    #[test]
    fn test_edit_student_details() {
        let mut students = create_students();
        students.edit_details(1, "Oluwagbemiga".to_string());
        assert_eq!(students.data[0].name, "Oluwagbemiga");
    }

    #[test]
    fn test_deactivate_student() {
        let mut students = create_students();
        students.deactivate(1);
        assert_eq!(students.data[0].isActive, IsActive::Inactive);
    }

    #[test]
    fn test_get_active_students() {
        let students = create_students();
        let active_students = students.get_active_students();
        assert!(!active_students.is_empty());
        assert_eq!(active_students[0].name, "Bestiee");
    }
}
