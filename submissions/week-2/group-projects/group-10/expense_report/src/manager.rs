use std::collections::HashMap;

use crate::errors::{EditError, RemoveError};
use crate::types::{Expense, ExpenseStatus};

pub struct ExpenseManager {
    pub expenses: HashMap<u32, Expense>,
    pub next_id: u32,
}

impl ExpenseManager {
    pub fn new() -> Self {
        Self {
            expenses: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_expense(&mut self, name: String, category: String, amount_str: String) -> Result<u32, String> {
        let amount: f64 = amount_str.parse().map_err(|_| "Invalid amount".to_string())?;
        let id = self.next_id;
        let expense = Expense {
            id,
            name,
            amount,
            category,
            status: ExpenseStatus::Pending,
        };
        self.expenses.insert(id, expense);
        self.next_id += 1;
        Ok(id)
    }

    pub fn view_expenses(&self) -> Vec<&Expense> {
        self.expenses.values().collect()
    }

    pub fn remove_expense(&mut self, id: u32) -> Result<(), RemoveError> {
        match self.expenses.get(&id) {
            Some(e) if matches!(e.status, ExpenseStatus::Approved | ExpenseStatus::Rejected) => {
                self.expenses.remove(&id);
                Ok(())
            }
            Some(_) => Err(RemoveError::CannotRemovePending),
            None => Err(RemoveError::NotFound),
        }
    }

    pub fn edit_expense(
        &mut self,
        id: u32,
        name: Option<String>,
        amount: Option<String>,
        category: Option<String>,
        status: Option<String>,
        confirm: bool,
    ) -> Result<(), EditError> {
        let expense = self.expenses.get(&id).cloned().ok_or(EditError::NotFound)?;

        if !confirm {
            return Err(EditError::Cancelled);
        }

        let amount_parsed = match amount {
            Some(ref a) => Some(a.parse::<f64>().map_err(|_| EditError::InvalidAmount)?),
            None => None,
        };

        let status_enum = match status {
            Some(s) => match s.to_lowercase().as_str() {
                "pending" => Some(ExpenseStatus::Pending),
                "approved" => Some(ExpenseStatus::Approved),
                "rejected" => Some(ExpenseStatus::Rejected),
                _ => return Err(EditError::InvalidStatus),
            },
            None => None,
        };

        let updated = Expense {
            id,
            name: name.unwrap_or(expense.name),
            amount: amount_parsed.unwrap_or(expense.amount),
            category: category.unwrap_or(expense.category),
            status: status_enum.unwrap_or(expense.status),
        };

        self.expenses.insert(id, updated);
        Ok(())
    }
}
