#[derive(Debug)]

pub struct ClassSystem {
    pub name: String,
    pub grade: StudentGrade,
    pub status: StudentStatus,
}

#[derive(Debug, Copy, Clone)]
pub enum StudentGrade {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Copy, Clone)]
pub enum StudentStatus {
    ACTIVE,
    NOTACTIVE,
    EXPELLED,
}

pub struct Students {
    pub student_list : Vec<ClassSystem>
}

impl Students {
    pub fn new() -> Students  {
        Students {
            student_list: Vec::new()
        }
    }

    // pub fn register_student(&mut self, student: ClassSystem) {
    //     self.student_list.push(student);
    // }

    pub fn register_student(&mut self, name: String, grade: StudentGrade, status: StudentStatus) {
        let student = ClassSystem {
            name,
            grade,
            status,
        };
        self.student_list.push(student);
    }


    pub fn view_all_students(&self) {
        for student in &self.student_list {
            println!("Name: {}, Grade: {:?}, Status: {:?}",
                     student.name, student.grade, student.status);
        }
    }

    // pub fn update_student(&mut self, name: String, new_grade: Option<StudentGrade>, new_status: Option<StudentStatus>) {
    //     for student in &mut self.student_list {
    //         if student.name == name {
    //             if let Some(grade) = new_grade {
    //                 student.grade = grade;
    //             }
    //             if let Some(status) = new_status {
    //                 student.status = status;
    //             }
    //         }
    //     }
    // }

    pub fn update_student(&mut self, name: String, grade: StudentGrade, status: StudentStatus) {
        for student in &mut self.student_list {
            if student.name  == name {
                student.grade = grade;
                student.status = status;
            }
        }
    }

    pub fn delete_student(&mut self, name: String) {
        self.student_list.retain(|student| student.name != name);
    }
}

fn main() {
    
}