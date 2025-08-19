#![no_std]

//! # Employee Management System
//! 
//! This contract implements a comprehensive employee management system that integrates
//! with SEP-41 tokens for salary payments. The system provides the following features:
//! 
//! - Employee registration with ranks and departments
//! - Rank-based salary calculation and promotion system
//! - Employee status management (active, suspended, terminated)
//! - Automated salary payments using SEP-41 tokens
//! - Administrative controls for institution management
//! 
//! ## Features
//! 
//! ### Employee Management
//! - Add new employees with initial rank and department
//! - Update employee information (name, department, base salary)
//! - Remove employees from the system
//! - Promote employees through rank hierarchy
//! - Suspend and reactivate employees
//! 
//! ### Rank System
//! - 9-tier rank hierarchy: Intern → Junior → Mid → Senior → Lead → Manager → Director → VP → CEO
//! - Salary multipliers based on rank (1x to 30x base salary)
//! - Promotion eligibility based on time intervals
//! 
//! ### Salary Management
//! - Integration with SEP-41 token contracts
//! - Rank-based salary calculation
//! - Individual and batch salary payments
//! - Salary payment tracking and events
//! 
//! ### Administrative Features
//! - Institution setup and configuration
//! - Admin role management
//! - Token contract integration
//! - Promotion interval configuration
//! 
//! ## Usage
//! 
//! 1. Initialize the contract with institution details and token contract
//! 2. Add employees with their initial ranks and departments
//! 3. Manage employee lifecycle (promotions, status changes, updates)
//! 4. Process salary payments using the integrated token system
//! 
//! ## Security Features
//! 
//! - Authorization required for all administrative operations
//! - Input validation on all parameters
//! - Proper event emission for transparency
//! - Protection against duplicate employee addresses

mod contract;
mod interface;
mod storage;
mod types;

pub use contract::EmployeeManagement;
pub use interface::{EmployeeManagementInterface, EmployeeManagementAdminInterface};
pub use types::{
    Employee, EmployeeRank, EmployeeStatus, Institution,
    EmployeeAddedEvent, EmployeePromotedEvent, EmployeeStatusChangedEvent, SalaryPaidEvent,
};

#[cfg(test)]
mod test;

// Re-export the contract for external use
pub use contract::EmployeeManagement as EmployeeManagementContract;
