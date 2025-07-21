// School management system
// Takes in name, grade
// Enum (active, inactive)

// Implementation function
// Add
// Update
// Delete
#[derive(Debug)]
pub struct StudentData {
    name: String,
    grade: String,
    isActive: ActiveState,
    id: i32,
}

#[derive(Debug)]
pub enum ActiveState {
    Active,
    InActive,
}

pub struct StudentsDataBase {
    pub data: Vec<StudentData>,
}

impl StudentsDataBase {
    fn new() -> StudentsDataBase {
        StudentsDataBase { data: Vec::new() }
    }

    pub fn add_student(&mut self, student: StudentData) {
        self.data.push(student);
    }

    pub fn update_student(&mut self, index: usize, student: StudentData) {
        if index < self.data.len() {
            self.data[index] = student;
        } else {
            println!("Index out of bounds");
        }
    }

    pub fn delete_student(&mut self, index: usize) {
        if index < self.data.len() {
            self.data.remove(index);
        } else {
            println!("Index out of bounds");
        }
    }

    pub fn update_student_grade(&mut self, id: i32, grade: String) {
        for student in &mut self.data {
            if student.id == id {
                student.grade = grade;
                break;
            }
        }
    }
}

#[cfg(test)]
#[test]
pub fn test_students_database() {
    let mut db = StudentsDataBase::new();

    let student1 = StudentData {
        name: String::from("Alice"),
        grade: String::from("A"),
        isActive: ActiveState::Active,
        id: 0,
    };

    let student2 = StudentData {
        name: String::from("Bob"),
        grade: String::from("B"),
        isActive: ActiveState::InActive,
        id: 1,
    };

    db.add_student(student1);
    db.add_student(student2);

    assert_eq!(db.data.len(), 2);

    db.update_student(
        0,
        StudentData {
            name: String::from("Alice Smith"),
            grade: String::from("A+"),
            isActive: ActiveState::Active,
            id: 0,
        },
    );

    assert_eq!(db.data[0].name, "Alice Smith");

    db.delete_student(1);
    assert_eq!(db.data.len(), 1);
    assert_eq!(db.data[0].name, "Alice Smith");

    db.update_student_grade(0, String::from("F"));

    // assert_eq!(db.data[0].isActive, ActiveState::Active);
    println!("All tests passed!");
}

fn main() {
    test_students_database();
}
