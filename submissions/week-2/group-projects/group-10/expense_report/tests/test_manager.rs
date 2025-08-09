use expense_report::errors::{EditError, RemoveError};
use expense_report::manager::ExpenseManager;
use expense_report::types::{ExpenseStatus, Expense};


  

    // Helper functions for test setup
    fn setup() -> ExpenseManager {
        ExpenseManager::new()
    }

    fn create_sample_expense(manager: &mut ExpenseManager) -> u32 {
        manager.add_expense(
            "Sample Expense".to_string(),
            "Test Category".to_string(),
            "10.00".to_string()
        ).unwrap()
    }

    fn create_expense_with_status(manager: &mut ExpenseManager, status: &str) -> u32 {
        let id = create_sample_expense(manager);
        manager.edit_expense(id, None, None, None, Some(status.to_string()), true).unwrap();
        id
    }

    fn create_approved_expense(manager: &mut ExpenseManager) -> u32 {
        create_expense_with_status(manager, "approved")
    }

    fn create_rejected_expense(manager: &mut ExpenseManager) -> u32 {
        create_expense_with_status(manager, "rejected")
    }

    fn create_multiple_expenses(manager: &mut ExpenseManager, count: u32) -> Vec<u32> {
        (0..count).map(|i| {
            manager.add_expense(
                format!("Expense {}", i + 1),
                format!("Category {}", i + 1),
                format!("{}.00", (i + 1) * 10)
            ).unwrap()
        }).collect()
    }

    fn assert_expense_fields(expense: &Expense, id: u32, name: &str, amount: f64, category: &str, status: ExpenseStatus) {
        assert_eq!(expense.id, id);
        assert_eq!(expense.name, name);
        assert_eq!(expense.amount, amount);
        assert_eq!(expense.category, category);
        assert_eq!(expense.status, status);
    }

    #[test]
    fn test_new_expense_manager() {
        let manager = setup();
        assert_eq!(manager.next_id, 1);
        assert_eq!(manager.expenses.len(), 0);
    }

    #[test]
    fn test_add_expense_success() {
        let mut manager = setup();
        
        let result = manager.add_expense(
            "Lunch".to_string(),
            "Food".to_string(),
            "12.50".to_string()
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(manager.next_id, 2);
        assert_eq!(manager.expenses.len(), 1);
        
        let expense = manager.expenses.get(&1).unwrap();
        assert_expense_fields(expense, 1, "Lunch", 12.50, "Food", ExpenseStatus::Pending);
    }

    #[test]
    fn test_add_expense_invalid_amount() {
        let mut manager = setup();
        
        let result = manager.add_expense(
            "Invalid".to_string(),
            "Test".to_string(),
            "not_a_number".to_string()
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid amount");
        assert_eq!(manager.expenses.len(), 0);
        assert_eq!(manager.next_id, 1); // Should not increment on error
    }

    #[test]
    fn test_add_multiple_expenses() {
        let mut manager = setup();
        let ids = create_multiple_expenses(&mut manager, 3);
        
        assert_eq!(ids, vec![1, 2, 3]);
        assert_eq!(manager.expenses.len(), 3);
        assert_eq!(manager.next_id, 4);
    }

    #[test]
    fn test_view_expenses_empty() {
        let manager = setup();
        let expenses = manager.view_expenses();
        assert_eq!(expenses.len(), 0);
    }

    #[test]
    fn test_view_expenses_with_data() {
        let mut manager = setup();
        create_multiple_expenses(&mut manager, 2);
        
        let expenses = manager.view_expenses();
        assert_eq!(expenses.len(), 2);
        
        // Check that we have both expenses (order may vary due to HashMap)
        let ids: Vec<u32> = expenses.iter().map(|e| e.id).collect();
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
    }

    #[test]
    fn test_remove_expense_approved() {
        let mut manager = setup();
        let id = create_approved_expense(&mut manager);
        
        let result = manager.remove_expense(id);
        assert!(result.is_ok());
        assert_eq!(manager.expenses.len(), 0);
    }

    #[test]
    fn test_remove_expense_rejected() {
        let mut manager = setup();
        let id = create_rejected_expense(&mut manager);
        
        let result = manager.remove_expense(id);
        assert!(result.is_ok());
        assert_eq!(manager.expenses.len(), 0);
    }

    #[test]
    fn test_remove_expense_pending() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.remove_expense(id);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RemoveError::CannotRemovePending));
        assert_eq!(manager.expenses.len(), 1); // Should still be there
    }

    #[test]
    fn test_remove_expense_not_found() {
        let mut manager = setup();
        
        let result = manager.remove_expense(999);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RemoveError::NotFound));
    }

    #[test]
    fn test_edit_expense_not_found() {
        let mut manager = setup();
        
        let result = manager.edit_expense(999, None, None, None, None, true);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EditError::NotFound));
    }

    #[test]
    fn test_edit_expense_not_confirmed() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, Some("New Name".to_string()), None, None, None, false);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EditError::Cancelled));
        
        // Verify expense wasn't changed
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.name, "Sample Expense");
    }

    #[test]
    fn test_edit_expense_name() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, Some("New Name".to_string()), None, None, None, true);
        assert!(result.is_ok());
        
        let expense = manager.expenses.get(&id).unwrap();
        assert_expense_fields(expense, id, "New Name", 10.00, "Test Category", ExpenseStatus::Pending);
    }

    #[test]
    fn test_edit_expense_amount_valid() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, None, Some("25.75".to_string()), None, None, true);
        assert!(result.is_ok());
        
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.amount, 25.75);
    }

    #[test]
    fn test_edit_expense_amount_invalid() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, None, Some("invalid".to_string()), None, None, true);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EditError::InvalidAmount));
        
        // Verify expense wasn't changed
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.amount, 10.00);
    }

    #[test]
    fn test_edit_expense_category() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, None, None, Some("New Category".to_string()), None, true);
        assert!(result.is_ok());
        
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.category, "New Category");
    }

    #[test]
    fn test_edit_expense_status_pending() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, None, None, None, Some("pending".to_string()), true);
        assert!(result.is_ok());
        
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.status, ExpenseStatus::Pending);
    }

    #[test]
    fn test_edit_expense_status_approved() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, None, None, None, Some("approved".to_string()), true);
        assert!(result.is_ok());
        
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.status, ExpenseStatus::Approved);
    }

    #[test]
    fn test_edit_expense_status_rejected() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, None, None, None, Some("rejected".to_string()), true);
        assert!(result.is_ok());
        
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.status, ExpenseStatus::Rejected);
    }

    #[test]
    fn test_edit_expense_status_case_insensitive() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, None, None, None, Some("APPROVED".to_string()), true);
        assert!(result.is_ok());
        
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.status, ExpenseStatus::Approved);
    }

    #[test]
    fn test_edit_expense_status_invalid() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(id, None, None, None, Some("invalid_status".to_string()), true);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EditError::InvalidStatus));
        
        // Verify expense wasn't changed
        let expense = manager.expenses.get(&id).unwrap();
        assert_eq!(expense.status, ExpenseStatus::Pending);
    }

    #[test]
    fn test_edit_expense_all_fields() {
        let mut manager = setup();
        let id = create_sample_expense(&mut manager);
        
        let result = manager.edit_expense(
            id,
            Some("Updated Name".to_string()),
            Some("50.25".to_string()),
            Some("Updated Category".to_string()),
            Some("approved".to_string()),
            true
        );
        assert!(result.is_ok());
        
        let expense = manager.expenses.get(&id).unwrap();
        assert_expense_fields(expense, id, "Updated Name", 50.25, "Updated Category", ExpenseStatus::Approved);
    }

    #[test]
    fn test_expense_status_debug_and_clone() {
        let status = ExpenseStatus::Pending;
        let cloned = status.clone();
        assert_eq!(status, cloned);
        
        // Test Debug formatting
        let debug_str = format!("{:?}", status);
        assert_eq!(debug_str, "Pending");
    }

    #[test]
    fn test_expense_debug_and_clone() {
        let expense = Expense {
            id: 1,
            name: "Test".to_string(),
            amount: 10.0,
            category: "Cat".to_string(),
            status: ExpenseStatus::Pending,
        };
        
        let cloned = expense.clone();
        assert_expense_fields(&cloned, 1, "Test", 10.0, "Cat", ExpenseStatus::Pending);
        
        // Test Debug formatting works
        let debug_str = format!("{:?}", expense);
        assert!(debug_str.contains("Test"));
    }

    #[test]
    fn test_error_types_debug() {
        // Test RemoveError Debug
        let remove_error = RemoveError::NotFound;
        let debug_str = format!("{:?}", remove_error);
        assert_eq!(debug_str, "NotFound");
        
        // Test EditError Debug
        let edit_error = EditError::InvalidAmount;
        let debug_str = format!("{:?}", edit_error);
        assert_eq!(debug_str, "InvalidAmount");
    }

