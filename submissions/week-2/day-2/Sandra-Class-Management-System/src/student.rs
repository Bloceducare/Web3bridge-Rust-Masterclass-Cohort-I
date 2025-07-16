use std::fmt;

/// Enum to track if a student is active or inactive
#[derive(Debug, Clone, PartialEq)]
pub enum StudentStatus {
    Active,
    Inactive,
}

impl fmt::Display for StudentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StudentStatus::Active => write!(f, "Active"),
            StudentStatus::Inactive => write!(f, "Inactive"),
        }
    }
}

/// Student struct containing name, grade, and status
#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grade: f64,
    pub status: StudentStatus,
}

impl Student {
    /// Create a new student
    pub fn new(id: u32, name: String, grade: f64) -> Self {
        Self {
            id,
            name,
            grade,
            status: StudentStatus::Active, // Default to active
        }
    }

    /// Update student's grade
    pub fn update_grade(&mut self, new_grade: f64) {
        self.grade = new_grade;
    }

    /// Update student's name
    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    /// Toggle student status between active and inactive
    pub fn toggle_status(&mut self) {
        self.status = match self.status {
            StudentStatus::Active => StudentStatus::Inactive,
            StudentStatus::Inactive => StudentStatus::Active,
        };
    }

    /// Set student status explicitly
    pub fn set_status(&mut self, status: StudentStatus) {
        self.status = status;
    }

    /// Check if student is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, StudentStatus::Active)
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}, Name: {}, Grade: {:.2}, Status: {}",
            self.id, self.name, self.grade, self.status
        )
    }
} 