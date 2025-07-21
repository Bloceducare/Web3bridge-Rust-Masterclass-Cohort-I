
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
