#[derive(Debug, Clone, PartialEq)]
pub enum ExpenseStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: u32,
    pub name: String,
    pub amount: f64,
    pub category: String,
    pub status: ExpenseStatus,
}
