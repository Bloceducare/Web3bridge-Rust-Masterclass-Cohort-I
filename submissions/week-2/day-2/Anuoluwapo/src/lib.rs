// pub struct Student {
//     pub id: u32,
//     pub name: String,
//     pub grade: String,
//     pub is_active: Status,
// }

// pub enum Status {
//     Active,
//     Inactive,
// }
// // id: u32;
// // pub enum Grade {
// //     Grade A,
// //     Grade B,
// //     Grade C,
// //     Grade D,
// //     Grade F,
// // }

// pub struct StudentDetails {
//     data: Vec<Student>,
//     incre_id: u32,
// }

// impl StudentDetails {
//     fn Self() {
//         StudentDetails {
//             data = Vec::new();
//             incre_id = 1;
//         }

//     }
//     pub fn register(&mut self, name: String, grade: String) -> Self {
//         let next_id = self.incre_id;
//         let student =  Student{
//             id: self.incre_id,
//             name,
//             grade,
//             is_active: Status::Active, 
//         };
//         self.incre_id += 1;
//         self.data.push(student)
//         next_id
//     }

//     pub fn edit(&mut self, new_name: Option<String>, new_grade: Option<String>) {
//         if let Some(name) = new_name {
//             self.name = name;
//         }
//         if let Some(grade) = new_grade {
//             self.grade = grade;
//         }
//     }

//     pub fn update_status(&mut self, id: u32, status: Status) {

//         match id {
//             self.id => {
//                 student.is_active = status;
//         }


//         // if id == id {
//         //     student.is_active = status
//         // }
//         // self.is_active = status;

//     }

//     pub fn delete(self) -> Self {
//         self
//     }

//     pub fn view(&self) -> String {
//         let status = match self.is_active {
//             Status::Active => "Active",
//             Status::Inactive => "Inactive",
//         };
//         format!("Name: {}, Grade: {}, Status: {}", self.name, self.grade, status)
//     }
// }

// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn setup() Student {
        
//     }

//     #[test]
//     fn test_register() {
//         let student = Class::register("John Doe".to_string(), 85);
//         assert_eq!(student.name, "John Doe");
//         assert_eq!(student.grade, 85);
//         match student.is_active {
//             Status::Active => assert!(true),
//             Status::Inactive => panic!("New student should be active by default"),
//         }
//     }

//     #[test]
//     fn test_edit_name() {
//         let mut student = Class::register("Jane Smith".to_string(), 75);
//         student.edit(Some("Jane Doe".to_string()), None);
//         assert_eq!(student.name, "Jane Doe");
//         assert_eq!(student.grade, 75); 
//     }

//     #[test]
//     fn test_edit_grade() {
//         let mut student = Class::register("Bob Johnson".to_string(), 80);
//         student.edit(None, Some(90));
//         assert_eq!(student.name, "Bob Johnson"); 
//         assert_eq!(student.grade, 90);
//     }

//     #[test]
//     fn test_edit_both() {
//         let mut student = Class::register("Alice".to_string(), 70);
//         student.edit(Some("Alice Wonderland".to_string()), Some(95));
//         assert_eq!(student.name, "Alice Wonderland");
//         assert_eq!(student.grade, 95);
//     }

//     #[test]
//     fn test_update_status_to_inactive() {
//         let mut student = Class::register("Test Student".to_string(), 100);
//         student.update_status(Status::Inactive);
//         match student.is_active {
//             Status::Inactive => assert!(true),
//             Status::Active => panic!("Status should be Inactive"),
//         }
//     }

//     #[test]
//     fn test_update_status_to_active() {
//         let mut student = Class::register("Test Student".to_string(), 100);
//         student.update_status(Status::Inactive); 
//         student.update_status(Status::Active); 
//         match student.is_active {
//             Status::Active => assert!(true),
//             Status::Inactive => panic!("Status should be Active"),
//         }
//     }

//     #[test]
//     fn test_delete() {
//         let student = Class::register("To Be Deleted".to_string(), 50);
//         let deleted_student = student.delete();
//         assert_eq!(deleted_student.name, "To Be Deleted");
//         assert_eq!(deleted_student.grade, 50);
//     }

//     #[test]
//     fn test_view_active() {
//         let student = Class::register("View Test".to_string(), 88);
//         let view_output = student.view();
//         assert!(view_output.contains("Name: View Test"));
//         assert!(view_output.contains("Grade: 88"));
//         assert!(view_output.contains("Status: Active"));
//     }

//     #[test]
//     fn test_view_inactive() {
//         let mut student = Class::register("View Test".to_string(), 88);
//         student.update_status(Status::Inactive);
//         let view_output = student.view();
//         assert!(view_output.contains("Status: Inactive"));
//     }
// }