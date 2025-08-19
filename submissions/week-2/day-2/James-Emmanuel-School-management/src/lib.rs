#[derive(Debug, Clone, PartialEq)]
pub enum StudentStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grade: u8,
    pub status: StudentStatus,
}

impl Student {
    pub fn new(id: u32, name: &str, grade: u8, status: StudentStatus) -> Self {
        Student {
            id,
            name: name.to_string(),
            grade,
            status,
        }
    }
}

#[derive(Debug)]
pub struct Classroom {
    students: Vec<Student>,
}

impl Classroom {
    pub fn new() -> Self {
        Classroom {
            students: Vec::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn get_all_students(&self) -> &Vec<Student> {
        &self.students
    }

    pub fn get_active_students(&self) -> Vec<&Student> {
        self.students
            .iter()
            .filter(|s| s.status == StudentStatus::Active)
            .collect()
    }

    pub fn get_student_by_name(&self, name: &str) -> Option<&Student> {
        self.students.iter().find(|s| s.name == name)
    }

    pub fn update_grade(&mut self, name: &str, new_grade: u8) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.name == name) {
            student.grade = new_grade;
            true
        } else {
            false
        }
    }

    pub fn set_status(&mut self, name: &str, status: StudentStatus) -> bool {
        if let Some(student) = self.students.iter_mut().find(|s| s.name == name) {
            student.status = status;
            true
        } else {
            false
        }
    }
}


#[test]
fn test_add_and_retrieve_students() {
    let mut class = Classroom::new();

    class.add_student(Student::new(100, "Bola", 85, StudentStatus::Active));
    class.add_student(Student::new(101, "Charles", 90, StudentStatus::Inactive));

    assert_eq!(class.get_all_students().len(), 2);

    let alice = class.get_student_by_name("Bola").unwrap();
    assert_eq!(alice.grade, 85);
    assert_eq!(alice.status, StudentStatus::Active);
}

#[test]
fn test_get_active_students() {
    let mut class = Classroom::new();

    class.add_student(Student::new(103, "Bola", 85, StudentStatus::Active));
    class.add_student(Student::new(104, "Charles", 90, StudentStatus::Inactive));
    class.add_student(Student::new(105, "Alex", 88, StudentStatus::Active));

    let active_students = class.get_active_students();
    assert_eq!(active_students.len(), 2);
    assert_eq!(active_students[0].name, "Bola");
    assert_eq!(active_students[1].name, "Charles");
}

#[test]
fn test_update_grade() {
    let mut class = Classroom::new();
    class.add_student(Student::new(106,"Dare", 70, StudentStatus::Active));

    let updated = class.update_grade("Dare", 95);
    assert!(updated);

    let daisy = class.get_student_by_name("Dare").unwrap();
    assert_eq!(daisy.grade, 95);
}

#[test]
fn test_set_status() {
    let mut class = Classroom::new();
    class.add_student(Student::new(109, "Eva", 60, StudentStatus::Active));

    let changed = class.set_status("Eva", StudentStatus::Inactive);
    assert!(changed);

let eve = class.get_student_by_name("Eva").unwrap();
    assert_eq!(eve.status, StudentStatus::Inactive);
}
