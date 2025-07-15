#[derive(Debug)]
struct Student {
    name: String,
    active: IsActive,
}

#[derive(Debug)]
struct StudentList {
    students: Vec<Student>,
}

impl StudentList {
    fn new() -> Self {
        StudentList { students: vec![] }
    }

    fn get_student(&self, index: usize) -> Option<&Student> {
        self.students.get(index)
    }

    fn all_students(&self) -> &Vec<Student> {
        &self.students
    }

    fn add_active_student(&mut self, name: String) {
        let student = Student {
            name,
            active: IsActive::Active,
        };
        self.students.push(student);
    }

    fn add(&mut self, name: String) {
        self.students.push(Student::new(name));
    }

    fn remove(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        } else {
            println!("No student found at index {}", index);
        }
    }
    fn edit(&mut self, index: usize, new_name: String, new_active: IsActive) {
        if index < self.students.len() {
            self.students[index].name = new_name;
            self.students[index].active = new_active;
        } else {
            println!("No student found at index {}", index);
        }
    }

    fn delete(&mut self, index: usize) {
        if index < self.students.len() {
            self.students.remove(index);
        } else {
            println!("No student found at index {}", index);
        }
    }
}

#[derive(Debug)]
enum IsActive {
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

fn main() {
    let mut student_list = StudentList::new();
    student_list.add("Gbemiga".to_string());
    student_list.add("Josh".to_string());
    println!("Hello, {:?}", student_list);

    student_list.edit(0, "Oluwagbemiga".to_string(), IsActive::Active);
    println!("Hello, {:#?}", student_list);
}
