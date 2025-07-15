#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_student() {
        let mut students = Vec::new();
        register_student(&mut students, "Test Student".to_string(), 8);

        assert_eq!(students.len(), 1);
        assert_eq!(students[0].name, "Test Student");
        assert_eq!(students[0].grade, 8);
        match students[0].status {
            Status::Active => {}
            _ => panic!("Status should be Active"),
        }
    }

    #[test]
    fn test_edit_student() {
        let mut students = vec![
            Student {
                name: "Original".to_string(),
                grade: 6,
                status: Status::Active,
            }
        ];

        edit_student(&mut students, 0, "Edited".to_string(), 9);

        assert_eq!(students[0].name, "Edited");
        assert_eq!(students[0].grade, 9);
    }

    #[test]
    fn test_update_status() {
        let mut students = vec![
            Student {
                name: "Alex".to_string(),
                grade: 10,
                status: Status::Active,
            }
        ];

        update_status(&mut students, 0, Status::Inactive);

        match students[0].status {
            Status::Inactive => {}
            _ => panic!("Status should be Inactive"),
        }
    }

    #[test]
    fn test_delete_student() {
        let mut students = vec![
            Student {
                name: "A".to_string(),
                grade: 5,
                status: Status::Active,
            },
            Student {
                name: "B".to_string(),
                grade: 6,
                status: Status::Active,
            }
        ];

        delete_student(&mut students, 0);

        assert_eq!(students.len(), 1);
        assert_eq!(students[0].name, "B");
    }

    #[test]
    fn test_view_students_output() {
        let students = vec![
            Student {
                name: "Viewer".to_string(),
                grade: 7,
                status: Status::Active,
            }
        ];

       
        view_students(&students);
    }
}
