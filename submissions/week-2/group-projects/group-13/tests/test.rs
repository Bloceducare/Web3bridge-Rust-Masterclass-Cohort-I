#[cfg(test)]
mod tests {
    use assignment::*;
    use super::*;

    fn sample_session_data() -> (String, String, String) {
        (
            "Blockchain 101".to_string(),
            "2025-07-21".to_string(),
            "Dr. Alice".to_string(),
        )
    }

    #[test]
    fn test_create_session() {
        let mut manager = TrainingSessionManager::new();
        let (course, date, trainer) = sample_session_data();

        let session_id = manager.create_session(course.clone(), date.clone(), trainer.clone());

        assert_eq!(session_id, 1);
        assert_eq!(manager.sessions.len(), 1);

        let session = manager.sessions.get(&session_id).unwrap();
        assert_eq!(session.course_name, course);
        assert_eq!(session.date, date);
        assert_eq!(session.trainer, trainer);
    }

    #[test]
    fn test_view_all_sessions_not_empty() {
        let mut manager = TrainingSessionManager::new();
        let (course, date, trainer) = sample_session_data();
        manager.create_session(course, date, trainer);

        assert!(!manager.sessions.is_empty());
        assert_eq!(manager.sessions.len(), 1);
    }

    #[test]
    fn test_delete_session() {
        let mut manager = TrainingSessionManager::new();
        let (course, date, trainer) = sample_session_data();
        let session_id = manager.create_session(course, date, trainer);

        assert!(manager.sessions.contains_key(&session_id));

        manager.delete_session(session_id);
        assert!(!manager.sessions.contains_key(&session_id));
        assert_eq!(manager.sessions.len(), 0);
    }

    #[test]
    fn test_edit_session_updates_fields() {
        let mut manager = TrainingSessionManager::new();
        let session_id = manager.create_session(
            "Intro to Rust".to_string(),
            "2025-07-22".to_string(),
            "Bob".to_string(),
        );

        manager.edit_session(
            session_id,
            "Advanced Rust".to_string(),
            "2025-08-10".to_string(),
            "Charlie".to_string(),
        );

        let session = manager.sessions.get(&session_id).unwrap();
        assert_eq!(session.course_name, "Advanced Rust");
        assert_eq!(session.date, "2025-08-10");
        assert_eq!(session.trainer, "Charlie");
    }

    #[test]
    fn test_edit_session_skips_empty_fields() {
        let mut manager = TrainingSessionManager::new();
        let session_id = manager.create_session(
            "Smart Contracts".to_string(),
            "2025-07-25".to_string(),
            "Dana".to_string(),
        );

        manager.edit_session(session_id, "".to_string(), "".to_string(), "Eve".to_string());

        let session = manager.sessions.get(&session_id).unwrap();
        assert_eq!(session.course_name, "Smart Contracts"); // unchanged
        assert_eq!(session.date, "2025-07-25");             // unchanged
        assert_eq!(session.trainer, "Eve");                 // updated
    }
}