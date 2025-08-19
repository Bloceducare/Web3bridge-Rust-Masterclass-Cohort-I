pub mod student;
pub mod student_manager;

pub use student::{Student, StudentStatus};
pub use student_manager::StudentManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let student = Student::new(1, "John Doe".to_string(), 85.5);
        assert_eq!(student.id, 1);
        assert_eq!(student.name, "John Doe");
        assert_eq!(student.grade, 85.5);
        assert_eq!(student.status, StudentStatus::Active);
        assert!(student.is_active());
    }

    #[test]
    fn test_student_status_operations() {
        let mut student = Student::new(1, "Jane Doe".to_string(), 92.0);
        
        // Test initial status
        assert!(student.is_active());
        
        // Test toggle status
        student.toggle_status();
        assert!(!student.is_active());
        assert_eq!(student.status, StudentStatus::Inactive);
        
        // Test set status explicitly
        student.set_status(StudentStatus::Active);
        assert!(student.is_active());
    }

    #[test]
    fn test_student_updates() {
        let mut student = Student::new(1, "Test Student".to_string(), 70.0);
        
        // Test grade update
        student.update_grade(85.0);
        assert_eq!(student.grade, 85.0);
        
        // Test name update
        student.update_name("Updated Name".to_string());
        assert_eq!(student.name, "Updated Name");
    }

    #[test]
    fn test_student_display() {
        let student = Student::new(1, "Alice".to_string(), 95.5);
        let display_str = format!("{}", student);
        assert!(display_str.contains("ID: 1"));
        assert!(display_str.contains("Name: Alice"));
        assert!(display_str.contains("Grade: 95.50"));
        assert!(display_str.contains("Status: Active"));
    }

    #[test]
    fn test_student_manager_creation() {
        let manager = StudentManager::new();
        assert_eq!(manager.total_students(), 0);
        assert_eq!(manager.active_students_count(), 0);
        assert_eq!(manager.inactive_students_count(), 0);
    }

    #[test]
    fn test_student_registration() {
        let mut manager = StudentManager::new();
        
        // Test successful registration
        let result = manager.register_student("Alice".to_string(), 85.0);
        assert!(result.is_ok());
        let id = result.unwrap();
        assert_eq!(id, 1);
        assert_eq!(manager.total_students(), 1);
        
        // Test second registration gets different ID
        let result2 = manager.register_student("Bob".to_string(), 75.0);
        assert!(result2.is_ok());
        let id2 = result2.unwrap();
        assert_eq!(id2, 2);
        assert_eq!(manager.total_students(), 2);
    }

    #[test]
    fn test_student_registration_validation() {
        let mut manager = StudentManager::new();
        
        // Test empty name
        let result = manager.register_student("".to_string(), 85.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
        
        // Test whitespace only name
        let result = manager.register_student("   ".to_string(), 85.0);
        assert!(result.is_err());
        
        // Test invalid grade (too low)
        let result = manager.register_student("Alice".to_string(), -5.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("between 0.0 and 100.0"));
        
        // Test invalid grade (too high)
        let result = manager.register_student("Bob".to_string(), 150.0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("between 0.0 and 100.0"));
    }

    #[test]
    fn test_student_editing() {
        let mut manager = StudentManager::new();
        let id = manager.register_student("Alice".to_string(), 85.0).unwrap();
        
        // Test editing name only
        let result = manager.edit_student(id, Some("Alice Smith".to_string()), None);
        assert!(result.is_ok());
        let student = manager.view_student(id).unwrap();
        assert_eq!(student.name, "Alice Smith");
        assert_eq!(student.grade, 85.0);
        
        // Test editing grade only
        let result = manager.edit_student(id, None, Some(90.0));
        assert!(result.is_ok());
        let student = manager.view_student(id).unwrap();
        assert_eq!(student.name, "Alice Smith");
        assert_eq!(student.grade, 90.0);
        
        // Test editing both
        let result = manager.edit_student(id, Some("Alice Johnson".to_string()), Some(95.0));
        assert!(result.is_ok());
        let student = manager.view_student(id).unwrap();
        assert_eq!(student.name, "Alice Johnson");
        assert_eq!(student.grade, 95.0);
    }

    #[test]
    fn test_student_editing_validation() {
        let mut manager = StudentManager::new();
        let id = manager.register_student("Alice".to_string(), 85.0).unwrap();
        
        // Test editing with empty name
        let result = manager.edit_student(id, Some("".to_string()), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
        
        // Test editing with invalid grade
        let result = manager.edit_student(id, None, Some(150.0));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("between 0.0 and 100.0"));
        
        // Test editing non-existent student
        let result = manager.edit_student(999, Some("Test".to_string()), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_student_status_update() {
        let mut manager = StudentManager::new();
        let id = manager.register_student("Alice".to_string(), 85.0).unwrap();
        
        // Test making student inactive
        let result = manager.update_student_status(id, StudentStatus::Inactive);
        assert!(result.is_ok());
        let student = manager.view_student(id).unwrap();
        assert!(!student.is_active());
        
        // Test making student active again
        let result = manager.update_student_status(id, StudentStatus::Active);
        assert!(result.is_ok());
        let student = manager.view_student(id).unwrap();
        assert!(student.is_active());
        
        // Test updating non-existent student
        let result = manager.update_student_status(999, StudentStatus::Active);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_student_deletion() {
        let mut manager = StudentManager::new();
        let id = manager.register_student("Alice".to_string(), 85.0).unwrap();
        
        assert_eq!(manager.total_students(), 1);
        
        // Test successful deletion
        let result = manager.delete_student(id);
        assert!(result.is_ok());
        let deleted_student = result.unwrap();
        assert_eq!(deleted_student.name, "Alice");
        assert_eq!(manager.total_students(), 0);
        
        // Test deleting non-existent student
        let result = manager.delete_student(999);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_student_viewing() {
        let mut manager = StudentManager::new();
        let id = manager.register_student("Alice".to_string(), 85.0).unwrap();
        
        // Test viewing existing student
        let result = manager.view_student(id);
        assert!(result.is_ok());
        let student = result.unwrap();
        assert_eq!(student.name, "Alice");
        assert_eq!(student.grade, 85.0);
        
        // Test viewing non-existent student
        let result = manager.view_student(999);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_view_all_students() {
        let mut manager = StudentManager::new();
        
        // Test empty list
        let students = manager.view_all_students();
        assert!(students.is_empty());
        
        // Add some students
        manager.register_student("Alice".to_string(), 85.0).unwrap();
        manager.register_student("Bob".to_string(), 75.0).unwrap();
        manager.register_student("Charlie".to_string(), 95.0).unwrap();
        
        let students = manager.view_all_students();
        assert_eq!(students.len(), 3);
        
        // Check if sorted by ID
        assert_eq!(students[0].id, 1);
        assert_eq!(students[1].id, 2);
        assert_eq!(students[2].id, 3);
    }

    #[test]
    fn test_view_active_inactive_students() {
        let mut manager = StudentManager::new();
        
        let id1 = manager.register_student("Alice".to_string(), 85.0).unwrap();
        let id2 = manager.register_student("Bob".to_string(), 75.0).unwrap();
        let id3 = manager.register_student("Charlie".to_string(), 95.0).unwrap();
        
        // Make Bob inactive
        manager.update_student_status(id2, StudentStatus::Inactive).unwrap();
        
        // Test active students
        let active_students = manager.view_active_students();
        assert_eq!(active_students.len(), 2);
        assert_eq!(active_students[0].id, id1);
        assert_eq!(active_students[1].id, id3);
        
        // Test inactive students
        let inactive_students = manager.view_inactive_students();
        assert_eq!(inactive_students.len(), 1);
        assert_eq!(inactive_students[0].id, id2);
    }

    #[test]
    fn test_student_counts() {
        let mut manager = StudentManager::new();
        
        assert_eq!(manager.total_students(), 0);
        assert_eq!(manager.active_students_count(), 0);
        assert_eq!(manager.inactive_students_count(), 0);
        
        let id1 = manager.register_student("Alice".to_string(), 85.0).unwrap();
        let _id2 = manager.register_student("Bob".to_string(), 75.0).unwrap();
        
        assert_eq!(manager.total_students(), 2);
        assert_eq!(manager.active_students_count(), 2);
        assert_eq!(manager.inactive_students_count(), 0);
        
        manager.update_student_status(id1, StudentStatus::Inactive).unwrap();
        
        assert_eq!(manager.total_students(), 2);
        assert_eq!(manager.active_students_count(), 1);
        assert_eq!(manager.inactive_students_count(), 1);
    }

    #[test]
    fn test_average_grade_calculations() {
        let mut manager = StudentManager::new();
        
        // Test empty manager
        assert_eq!(manager.average_grade(), None);
        assert_eq!(manager.average_grade_active(), None);
        
        // Add students
        let _id1 = manager.register_student("Alice".to_string(), 80.0).unwrap();
        let _id2 = manager.register_student("Bob".to_string(), 90.0).unwrap();
        let id3 = manager.register_student("Charlie".to_string(), 70.0).unwrap();
        
        // Test overall average
        let avg = manager.average_grade().unwrap();
        assert!((avg - 80.0).abs() < 0.01); // 240 / 3 = 80
        
        // Make one student inactive
        manager.update_student_status(id3, StudentStatus::Inactive).unwrap();
        
        // Test active average (should be 85.0 for Alice and Bob)
        let active_avg = manager.average_grade_active().unwrap();
        assert!((active_avg - 85.0).abs() < 0.01); // (80 + 90) / 2 = 85
        
        // Overall average should still be 80.0
        let overall_avg = manager.average_grade().unwrap();
        assert!((overall_avg - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_average_grade_with_all_inactive() {
        let mut manager = StudentManager::new();
        
        let id1 = manager.register_student("Alice".to_string(), 80.0).unwrap();
        let id2 = manager.register_student("Bob".to_string(), 90.0).unwrap();
        
        // Make all students inactive
        manager.update_student_status(id1, StudentStatus::Inactive).unwrap();
        manager.update_student_status(id2, StudentStatus::Inactive).unwrap();
        
        // Active average should be None
        assert_eq!(manager.average_grade_active(), None);
        
        // Overall average should still work
        let overall_avg = manager.average_grade().unwrap();
        assert!((overall_avg - 85.0).abs() < 0.01); // (80 + 90) / 2 = 85
    }

    #[test]
    fn test_student_manager_default() {
        let manager = StudentManager::default();
        assert_eq!(manager.total_students(), 0);
    }

    #[test]
    fn test_student_status_display() {
        assert_eq!(format!("{}", StudentStatus::Active), "Active");
        assert_eq!(format!("{}", StudentStatus::Inactive), "Inactive");
    }

    #[test]
    fn test_integration_workflow() {
        let mut manager = StudentManager::new();
        
        // Register multiple students
        let _alice_id = manager.register_student("Alice Johnson".to_string(), 92.5).unwrap();
        let bob_id = manager.register_student("Bob Smith".to_string(), 78.0).unwrap();
        let charlie_id = manager.register_student("Charlie Brown".to_string(), 85.5).unwrap();
        
        // Edit a student
        manager.edit_student(bob_id, None, Some(82.0)).unwrap();
        
        // Update status
        manager.update_student_status(charlie_id, StudentStatus::Inactive).unwrap();
        
        // Verify final state
        assert_eq!(manager.total_students(), 3);
        assert_eq!(manager.active_students_count(), 2);
        assert_eq!(manager.inactive_students_count(), 1);
        
        let bob = manager.view_student(bob_id).unwrap();
        assert_eq!(bob.grade, 82.0);
        
        let charlie = manager.view_student(charlie_id).unwrap();
        assert!(!charlie.is_active());
        
        // Calculate averages
        let overall_avg = manager.average_grade().unwrap();
        assert!((overall_avg - 86.67).abs() < 0.01); // (92.5 + 82.0 + 85.5) / 3
        
        let active_avg = manager.average_grade_active().unwrap();
        assert!((active_avg - 87.25).abs() < 0.01); // (92.5 + 82.0) / 2
    }
} 