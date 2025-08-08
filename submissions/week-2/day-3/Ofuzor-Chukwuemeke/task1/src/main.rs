use task1::{Employee,Store};


fn main() {
    let employee = Employee::KITCHEN_STAFF;
    let store = Store {
        employee,
        terminated: true,
    };
}

#[cfg(test)]
mod testss {
    use super::*;

    fn initialize() {
        let employee = Employee::KITCHEN_STAFF;
        let store = Store {
            employee,
            terminated: true,
        };
        // assert_eq!(store.employee == Employee::TECHNICAL_SUPERVISORS);
    }
}
