/*
A simple class management system in Rust - TODO
A class management system that has the name of the student, grade, enum that tracks if student is active or not.

Have the following functions:
•⁠  ⁠Function to register student
•⁠  ⁠Edit
•⁠  ⁠Update 
•⁠  ⁠Delete functions
•⁠  ⁠View function
 */

#derive[debug(clone)]
pub enum Status {
    Active,
    Inactive,
}
#derive[debug(clone)]
pud struct Students {
    name: String,
    grade: u8,
    status: status,
}

pub struct Class {
   pub students: Vec<Students>,
}

impl Class {
    // create an empty class
    pub fn initialize() -> Self {
        Class {
            students: Vec::new(),
        }
    }
    /// Registers a new student with the given `name`, `grade`, and `status`.
    /// Returns the index of the newly registered student.
    /// The `name` and `grade` parameters are required, while `status` is optional and defaults to `Active`.
    /// If the `status` is not provided, it will default to `Active`.
    /// The `name` and `grade` parameters must not be empty or negative.
    /// If the `name` is empty or the `grade` is negative,
    /// the function will panic with a message indicating the error.
    /// The `status` parameter can be either `Active` or `Inactive`.
    /// If the `status` is not provided, it will default to `Active`.
    pub fn register_student(&mut self, name: String, grade: u8, status: Status) -> usize {
         let student = Student { name, grade, status };
        self.students.push(student);
        self.students.len() - 1 // return the index of the new student
    }

    
    /// Edits the student at `index` with new values.
    /// Returns `true` if the edit was successful, `false` otherwise.
    /// The `name`, `grade`, and `status` parameters are required.
    pub fn edit_student(&mut self, index: usize, name: String, grade: u8, status: Status) -> bool {
        if let Some(student) = self.students.get_mut(index) {
            student.name = new_name;
            student.grade = new_grade;
            student.status = new_status;
            true
        } else {
            false
        }   
    }

    
    /// Updates the student at `index` with new values.
    /// Returns `true` if the update was successful, `false` otherwise.
    /// The `name`, `grade`, and `status` parameters are optional.
    /// If a parameter is `None`, the corresponding field will not be updated.
    pub fn update_student( &mut self,
        index: usize,
        name: Option<String>,
        grade: Option<u8>,
        status: Option<Status>,) -> bool {
       if let Some(student) = self.students.get_mut(index) {
            if let Some(n) = name {
                student.name = n;
            }
            if let Some(g) = grade {
                student.grade = g;
            }
            if let Some(s) = status {
                student.status = s;
            }
            true
        } else {
            false
        }
    }

    /// Deletes the student at `index`.
     pub fn delete_student(&mut self, index: usize) -> bool {
        if index < self.students.len() {
            self.students.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns an immutable reference to the student at `index`.
    /// Panics on out-of-bounds access.
    pub fn view_student(&self, index: usize) -> &Student {
        &self.students[index]
    }

    /// Returns an immutable slice of all students.
    pub fn view_all(&self) -> &[Student] {
        &self.students
    }
}


