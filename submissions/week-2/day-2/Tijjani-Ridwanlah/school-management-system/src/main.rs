#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Status {
    Active,
    Inactive,
}

#[derive(Clone, Debug)]
pub struct StudentProfile {
    pub student_name: String,
    pub grade: u8,
    pub student_status: Status,
}

pub struct StudentRegister {
    registry: Vec<StudentProfile>,
}

impl StudentRegister {
    fn new() -> Self {
        Self {
            registry: Vec::new(),
        }
    }

    pub fn register_student(&mut self, student_name: String, grade: u8, student_status: Status) {
        let student = StudentProfile {
            student_name,
            grade,
            student_status,
        };

        self.registry.push(student);
    }

    pub fn update_student(
        &mut self,
        current_name: String,
        new_name: String,
        grade: u8,
        status: Status,
    ) -> Result<(), String> {
        let update = self
            .registry
            .iter_mut()
            .find(|update| update.student_name == current_name);

        match update {
            Some(e) => {
                e.student_name = new_name;
                e.grade = grade;
                e.student_status = status;
                Ok(())
            }
            None => Err(format!("Student {} is not found", current_name)),
        }
    }

    pub fn remove_student(&mut self, name: String) -> Result<(), String> {
        let student = self
            .registry
            .iter()
            .position(|student| student.student_name == name);
        match student {
            Some(name) => {
                self.registry.remove(name);
                Ok(())
            }
            None => Err(format!("Student {} is not found", name)),
        }
    }

    pub fn view_students(&self) {
        for student in &self.registry {
            println!("{:?}", student);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut register = StudentRegister::new();

        assert!(register.registry.len() == 0);

        let name = "Abdul".to_string();
        let grade = 87;
        let statue = Status::Active;

        register.register_student(name, grade, statue);

        assert!(register.registry.len() == 1);

        let name = "Rasaq".to_string();
        let grade = 70;
        let statue = Status::Active;

        register.register_student(name, grade, statue);

        assert!(register.registry.len() == 2);

        let name = "Taju".to_string();
        let grade = 87;
        let statue = Status::Inactive;

        register.register_student(name, grade, statue);

        assert!(register.registry.len() == 3);
    }

    #[test]
    fn test_update_student() {
        let mut register = StudentRegister::new();

        assert!(register.registry.len() == 0);

        let name = "Abdul".to_string();
        let grade = 87;
        let status = Status::Active;

        register.register_student(name.clone(), grade, status);

        assert!(register.registry.len() == 1);

        let updated_name = format!("{} Muhammed", name);
        let updated_grade = 70;
        let updated_status = Status::Inactive;

        register.update_student(name, updated_name.clone(), updated_grade, updated_status);

        assert!(register.registry.len() == 1);

        let student = register
            .registry
            .iter()
            .find(|s| s.student_name == updated_name);
        assert!(student.is_some());
        assert_eq!(student.unwrap().grade, updated_grade);
        assert!(matches!(student.unwrap().student_status, Status::Inactive));
    }

    #[test]
    fn test_remove_student() {
        let mut register = StudentRegister::new();

        assert!(register.registry.len() == 0);

        let name = "Abdul".to_string();
        let grade = 87;
        let statue = Status::Active;

        register.register_student(name, grade, statue);

        assert!(register.registry.len() == 1);

        let name = "Rasaq".to_string();
        let grade = 70;
        let statue = Status::Active;

        register.register_student(name, grade, statue);

        assert!(register.registry.len() == 2);

        let name = "Taju".to_string();
        let grade = 87;
        let statue = Status::Inactive;

        register.register_student(name.clone(), grade, statue);

        assert!(register.registry.len() == 3);

        register.remove_student(name);

        assert!(register.registry.len() == 2);
    }

    #[test]
    fn test_view_student() {
        let mut register = StudentRegister::new();

        assert!(register.registry.len() == 0);

        let name = "Abdul".to_string();
        let grade = 87;
        let statue = Status::Active;

        register.register_student(name, grade, statue);

        assert!(register.registry.len() == 1);

        let name = "Rasaq".to_string();
        let grade = 70;
        let statue = Status::Active;

        register.register_student(name, grade, statue);

        assert!(register.registry.len() == 2);

        let name = "Taju".to_string();
        let grade = 87;
        let statue = Status::Inactive;

        register.register_student(name.clone(), grade, statue);

        assert!(register.registry.len() == 3);

        register.view_students();

        assert!(register.registry.len() == 3);
    }
}
