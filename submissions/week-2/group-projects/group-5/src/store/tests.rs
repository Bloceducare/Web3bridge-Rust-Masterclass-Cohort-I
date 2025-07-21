#[cfg(test)]
mod tests {
    use super::super::BudgetStore;
    use std::fs;

    const TEST_FILE: &str = "test_budgets.json";

    fn clean() {
        let _ = fs::remove_file(TEST_FILE);
    }

    fn setup_store() -> BudgetStore {
        clean();
        BudgetStore {
            budgets: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_add_budget() {
        let mut store = setup_store();
        store.add("Marketing".into(), 5000.0);
        assert_eq!(store.budgets.get("Marketing"), Some(&5000.0));
    }

    #[test]
    fn test_edit_amount() {
        let mut store = setup_store();
        store.add("IT".into(), 1000.0);
        store.edit_amount("IT".into(), 3000.0);
        assert_eq!(store.budgets.get("IT"), Some(&3000.0));
    }

    #[test]
    fn test_remove_budget() {
        let mut store = setup_store();
        store.add("Finance".into(), 2000.0);
        store.remove("Finance".into());
        assert!(store.budgets.get("Finance").is_none());
    }

    #[test]
    fn test_edit_name() {
        let mut store = setup_store();
        store.add("R&D".into(), 800.0);
        store.edit_name("R&D".into(), "Research".into());
        assert!(store.budgets.get("R&D").is_none());
        assert_eq!(store.budgets.get("Research"), Some(&800.0));
    }
}
