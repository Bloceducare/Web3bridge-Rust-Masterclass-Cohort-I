pub struct Student {
    pub name: String,
    pub grade: u8,
    pub student_status: StudentStatus,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StudentStatus {
    NotActive,
    Active,
}

impl StudentStatus {
    pub fn check_variants(self) -> Self {
        match self {
            Self::Active => Self::Active,
            Self::NotActive => Self::Active,
        }
    }
}

impl Student {
    pub fn new(name: String, grade: u8) -> Self {
        Self {
            name: name,
            grade: grade,
            student_status: StudentStatus::Active,
        }
    }

    pub fn update_student_status(&mut self, status: StudentStatus) -> bool {
        self.student_status = status.check_variants();
        true
    }

    pub fn update_student_grade(&mut self, grade: u8) -> bool {
        self.grade = grade;
        true
    }
}
