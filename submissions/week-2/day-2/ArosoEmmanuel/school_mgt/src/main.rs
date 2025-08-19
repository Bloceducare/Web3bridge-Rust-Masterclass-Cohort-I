fn main() {
    
}


#[derive(Clone)]
struct StudentDB {
    name: String,
    class: String,
    age: u8,
    sex: Gender,
    currently_enrolled: Enrolled,
}

struct School {
    pub students: Vec<StudentDB>
}

#[derive(PartialEq, Clone)]
enum Enrolled {
    False,
    True,
}

#[derive(Clone)]
enum Gender {
    Male,
    Female,
    Other,
}

impl School {
    pub fn new () -> Self {
        School {
            students: Vec::new()
        }
    }

    pub fn add_student(&mut self, name: String, class: String, age: u8, gender: Gender) {
        let student = StudentDB {
            name,
            class,
            age,
            sex: gender,
            currently_enrolled: Enrolled::True,
        };

        self.students.push(student);
    }


    pub fn remove_student(&mut self, name: &str) {
        self.students.retain(|student| student.name != name);
    }

    pub fn get_students(&self) -> Vec<StudentDB> {
        self.students.to_vec()
    }

    // pub fn unenroll_student(&mut self, index: usize) {
    //     let mut student = self.students.get(index).unwrap();
    //     if student.currently_enrolled == Enrolled::False {
    //         return;
    //     }
    //     student.currently_enrolled = Enrolled::False;
    //     self.students.push(student.clone());
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_student() {
        let mut school = School::new();
        school.add_student("Alice".to_string(), "10th Grade".to_string);
    }
}