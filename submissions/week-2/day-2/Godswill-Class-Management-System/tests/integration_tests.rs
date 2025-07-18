use godswill_class_management_system::{ClassManagementSystem, Student, StudentStatus};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut cms = ClassManagementSystem::new();
        
        let id = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        assert_eq!(id, 1);
        
        let id2 = cms.register_student("Jane Smith".to_string(), "B".to_string()).unwrap();
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_register_student_with_empty_name() {
        let mut cms = ClassManagementSystem::new();
        
        let result = cms.register_student("".to_string(), "A".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn test_register_student_with_empty_grade() {
        let mut cms = ClassManagementSystem::new();
        
        let result = cms.register_student("John Doe".to_string(), "".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Grade cannot be empty");
    }

    #[test]
    fn test_view_student() {
        let mut cms = ClassManagementSystem::new();
        let id = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.name, "John Doe");
        assert_eq!(student.grade, "A");
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_view_nonexistent_student() {
        let cms = ClassManagementSystem::new();
        let result = cms.view_student(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_edit_student() {
        let mut cms = ClassManagementSystem::new();
        let id = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        
        let result = cms.edit_student(
            id,
            Some("John Smith".to_string()),
            Some("B".to_string()),
            Some(StudentStatus::Inactive)
        );
        assert!(result.is_ok());
        
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.name, "John Smith");
        assert_eq!(student.grade, "B");
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_edit_student_partial() {
        let mut cms = ClassManagementSystem::new();
        let id = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        
        let result = cms.edit_student(id, Some("John Smith".to_string()), None, None);
        assert!(result.is_ok());
        
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.name, "John Smith");
        assert_eq!(student.grade, "A");
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_edit_nonexistent_student() {
        let mut cms = ClassManagementSystem::new();
        let result = cms.edit_student(999, Some("John".to_string()), None, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Student not found");
    }

    #[test]
    fn test_update_student() {
        let mut cms = ClassManagementSystem::new();
        let id = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        
        let result = cms.update_student(
            id,
            "John Smith".to_string(),
            "B".to_string(),
            StudentStatus::Inactive
        );
        assert!(result.is_ok());
        
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.name, "John Smith");
        assert_eq!(student.grade, "B");
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_update_nonexistent_student() {
        let mut cms = ClassManagementSystem::new();
        let result = cms.update_student(999, "John".to_string(), "A".to_string(), StudentStatus::Active);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Student not found");
    }

    #[test]
    fn test_delete_student() {
        let mut cms = ClassManagementSystem::new();
        let id = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        
        let result = cms.delete_student(id);
        assert!(result.is_ok());
        
        let deleted_student = result.unwrap();
        assert_eq!(deleted_student.name, "John Doe");
        
        let student = cms.view_student(id);
        assert!(student.is_none());
    }

    #[test]
    fn test_delete_nonexistent_student() {
        let mut cms = ClassManagementSystem::new();
        let result = cms.delete_student(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Student not found");
    }

    #[test]
    fn test_view_all_students() {
        let mut cms = ClassManagementSystem::new();
        cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        cms.register_student("Jane Smith".to_string(), "B".to_string()).unwrap();
        
        let students = cms.view_all_students();
        assert_eq!(students.len(), 2);
    }

    #[test]
    fn test_view_all_students_empty() {
        let cms = ClassManagementSystem::new();
        let students = cms.view_all_students();
        assert_eq!(students.len(), 0);
    }

    #[test]
    fn test_get_students_by_status() {
        let mut cms = ClassManagementSystem::new();
        let id1 = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        let id2 = cms.register_student("Jane Smith".to_string(), "B".to_string()).unwrap();
        
        cms.edit_student(id2, None, None, Some(StudentStatus::Inactive)).unwrap();
        
        let active_students = cms.get_students_by_status(&StudentStatus::Active);
        let inactive_students = cms.get_students_by_status(&StudentStatus::Inactive);
        
        assert_eq!(active_students.len(), 1);
        assert_eq!(inactive_students.len(), 1);
        assert_eq!(active_students[0].name, "John Doe");
        assert_eq!(inactive_students[0].name, "Jane Smith");
    }

    #[test]
    fn test_get_students_by_grade() {
        let mut cms = ClassManagementSystem::new();
        cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        cms.register_student("Jane Smith".to_string(), "A".to_string()).unwrap();
        cms.register_student("Bob Johnson".to_string(), "B".to_string()).unwrap();
        
        let grade_a_students = cms.get_students_by_grade("A");
        let grade_b_students = cms.get_students_by_grade("B");
        
        assert_eq!(grade_a_students.len(), 2);
        assert_eq!(grade_b_students.len(), 1);
    }

    #[test]
    fn test_student_count() {
        let mut cms = ClassManagementSystem::new();
        assert_eq!(cms.student_count(), 0);
        
        cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        assert_eq!(cms.student_count(), 1);
        
        cms.register_student("Jane Smith".to_string(), "B".to_string()).unwrap();
        assert_eq!(cms.student_count(), 2);
        
        cms.delete_student(1).unwrap();
        assert_eq!(cms.student_count(), 1);
    }

    #[test]
    fn test_student_status_from_string() {
        assert_eq!(StudentStatus::from_string("active"), Some(StudentStatus::Active));
        assert_eq!(StudentStatus::from_string("ACTIVE"), Some(StudentStatus::Active));
        assert_eq!(StudentStatus::from_string("inactive"), Some(StudentStatus::Inactive));
        assert_eq!(StudentStatus::from_string("INACTIVE"), Some(StudentStatus::Inactive));
        assert_eq!(StudentStatus::from_string("invalid"), None);
    }

    #[test]
    fn test_student_new() {
        let student = Student::new(1, "John Doe".to_string(), "A".to_string());
        assert_eq!(student.id, 1);
        assert_eq!(student.name, "John Doe");
        assert_eq!(student.grade, "A");
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_student_update() {
        let mut student = Student::new(1, "John Doe".to_string(), "A".to_string());
        student.update("John Smith".to_string(), "B".to_string(), StudentStatus::Inactive);
        
        assert_eq!(student.name, "John Smith");
        assert_eq!(student.grade, "B");
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_student_edit() {
        let mut student = Student::new(1, "John Doe".to_string(), "A".to_string());
        student.edit(Some("John Smith".to_string()), None, Some(StudentStatus::Inactive));
        
        assert_eq!(student.name, "John Smith");
        assert_eq!(student.grade, "A");
        assert_eq!(student.status, StudentStatus::Inactive);
    }

    #[test]
    fn test_update_student_status() {
        let mut cms = ClassManagementSystem::new();
        let id = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        
        // Test setting to Inactive
        cms.update_student_status(id, StudentStatus::Inactive).unwrap();
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.status, StudentStatus::Inactive);
        
        // Test setting to Active
        cms.update_student_status(id, StudentStatus::Active).unwrap();
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_toggle_student_status() {
        let mut cms = ClassManagementSystem::new();
        let id = cms.register_student("John Doe".to_string(), "A".to_string()).unwrap();
        
        // Initial status should be Active
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.status, StudentStatus::Active);
        
        // Toggle to Inactive
        cms.toggle_student_status(id).unwrap();
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.status, StudentStatus::Inactive);
        
        // Toggle back to Active
        cms.toggle_student_status(id).unwrap();
        let student = cms.view_student(id).unwrap();
        assert_eq!(student.status, StudentStatus::Active);
    }

    #[test]
    fn test_toggle_student_status_nonexistent() {
        let mut cms = ClassManagementSystem::new();
        let result = cms.toggle_student_status(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Student not found");
    }
}