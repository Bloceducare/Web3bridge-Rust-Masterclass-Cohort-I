fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Clone)]
enum ActivesStatus{
    Active,
    NotActive,
}

#[derive(Debug, Clone)]
struct Student {
    name: String,
    grade: f64,
    status:ActivesStatus,
}

#[derive(Debug, Clone)]
struct ClassOfStudents{
    class_students: Vec<Student>,
}

impl ClassOfStudents {

    fn init_class ()-> ClassOfStudents{
        ClassOfStudents{
            class_students: Vec::new()
        }
    }

    fn create_student (&mut self, student: Student) {
        self.class_students.push(student);
    }

    fn get_all_students (&self) -> Vec<Student>{
        self.class_students.to_vec()
    }

    fn get_each_student (&self, index: usize) -> Student{
        if self.class_students.len() > index {
            self.class_students[index].clone()
        } else {
            panic!("student does not exist");
        }
    }

    fn update_student (&mut self, index:usize, student: Student){
         if self.class_students.len() > index {
            self.class_students[index] = student;
        } else {
            panic!("student does not exist");
        }
    }

    fn delete_student (&mut self, index:usize){
        if self.class_students.len() > index {
            self.class_students.remove(index);
        } else {
            panic!("student does not exist");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_student() {
        let mut class = ClassOfStudents::init_class();
        assert!(class.class_students.len() == 0);

        let student = Student {
            name: "Sogo".to_string(),
            grade: 4.5,
            status:ActivesStatus::Active
        };

        class.create_student(student);
        assert!(class.class_students.len() == 1);
    }

    
  
}