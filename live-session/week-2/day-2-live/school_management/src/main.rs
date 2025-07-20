#[derive(Debug, Clone, PartialEq)]
enum StudentStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone)]
struct StudentDetails {
    id: u32,
    name: String,
    status: StudentStatus,
}

struct StudentsData {
    data: Vec<StudentDetails>,
    next_id: u32,
}

impl StudentsData {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            next_id: 1,
        }
    }

    fn register(&mut self, name: String) -> u32 {
        let present_id = self.next_id;
        let student = StudentDetails {
            id: present_id,
            name,
            status: StudentStatus::Active,
        };
        self.next_id += 1;
        self.data.push(student);
        present_id
    }

    fn evict_student2(&mut self, id: u32) -> bool {
        if let Some(student_detail) = self.data.iter_mut().position(|student| student.id == id) {
            self.data.remove(student_detail);
            true
        } else {
            false
        }
    }

    fn evict_student(&mut self, id: u32) {
        self.data.retain(|student_id| student_id.id != id);
    }

    fn get_student(&self, id: u32) -> &StudentDetails {
        self.data
            .iter()
            .find(|student_id| student_id.id == id)
            .unwrap()
    }

    fn update_student(&mut self, id: u32, new_name: String) -> bool {
        if let Some(student) = self.data.iter_mut().find(|stu_id| stu_id.id == id) {
            student.name = new_name;
            true
        } else {
            false
        }
    }

    fn update_status(&mut self, id: u32, new_status: StudentStatus) -> bool {
        if let Some(student) = self.data.iter_mut().find(|student_id| student_id.id == id) {
            student.status = new_status;
            true
        } else {
            false
        }
    }

    fn get_all_student(&self) -> &Vec<StudentDetails> {
        &self.data
    }
}
fn main() {
    println!("Hello");
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> StudentsData {
        let mut student_data = StudentsData::new();

        student_data.register("Joshua".to_string());
        student_data.register("Armolas".to_string());
        student_data
    }

    #[test]
    fn test_register_student() {
        let data = setup();
        assert_eq!(data.data.len(), 2);
        assert_eq!(data.data[0].name, "Joshua".to_string());
        assert_eq!(data.data[1].name, "Armolas".to_string());
        assert_eq!(data.data[1].id, 2);
        assert_eq!(data.data[1].status, StudentStatus::Active);
    }

    #[test]
    fn test_get_student_by_id() {
        let data = setup();
        let student_details = data.get_student(1);
        assert_eq!(student_details.name, "Joshua".to_string());
    }

    #[test]
    fn test_update_student_name() {
        let mut data = setup();
        let update_student = data.update_student(1, "Josh".to_string());
        assert!(update_student);
        let new_name = data.get_student(1);
        assert_eq!(new_name.name, "Josh".to_string());
    }

    #[test]
    fn test_evict_student() {
        let mut data = setup();
        data.evict_student(1);
        assert_eq!(data.data.len(), 1);
    }

    #[test]
    fn test_get_all_students() {
        let data = setup();
        let students = data.get_all_student();
        assert_eq!(students.len(), 2);
    }

    #[test]
    fn test_update_student_status() {
        let mut data = setup();

        let updated_status = data.update_status(2, StudentStatus::Inactive);
        assert!(updated_status);
        assert_eq!(data.get_student(2).status, StudentStatus::Inactive);
    }
}
