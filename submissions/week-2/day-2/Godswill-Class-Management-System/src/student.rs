#[derive(Debug, Clone, PartialEq)]
pub enum StudentStatus {
    Active,
    Inactive,
}

impl StudentStatus {
    pub fn from_string(status: &str) -> Option<Self> {
        match status.to_lowercase().as_str() {
            "active" => Some(StudentStatus::Active),
            "inactive" => Some(StudentStatus::Inactive),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grade: String,
    pub status: StudentStatus,
}

impl Student {
    pub fn new(id: u32, name: String, grade: String) -> Self {
        Student {
            id,
            name,
            grade,
            status: StudentStatus::Active,
        }
    }

    pub fn update(&mut self, name: String, grade: String, status: StudentStatus) {
        self.name = name;
        self.grade = grade;
        self.status = status;
    }

    pub fn edit(&mut self, name: Option<String>, grade: Option<String>, status: Option<StudentStatus>) {
        if let Some(new_name) = name {
            self.name = new_name;
        }
        if let Some(new_grade) = grade {
            self.grade = new_grade;
        }
        if let Some(new_status) = status {
            self.status = new_status;
        }
    }
}