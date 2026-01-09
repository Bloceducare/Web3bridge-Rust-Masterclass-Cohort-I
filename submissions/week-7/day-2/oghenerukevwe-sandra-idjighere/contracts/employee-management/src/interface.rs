use soroban_sdk::{Address, Env, String, Vec};
use crate::types::{Employee, EmployeeRank, Institution};

/// Employee Management Interface
/// 
/// This trait defines the interface for managing employees in an institution
/// with integration to SEP-41 token for salary payments.
pub trait EmployeeManagementInterface {
    /// Initialize the employee management system
    ///
    /// # Arguments
    ///
    /// * `admin` - The admin address for the institution
    /// * `institution_name` - Name of the institution
    /// * `token_contract` - Address of the SEP-41 token contract for salary payments
    /// * `base_salary_amount` - Base salary amount (will be multiplied by rank multiplier)
    /// * `min_promotion_interval` - Minimum ledger sequences between promotions
    fn initialize(
        env: Env,
        admin: Address,
        institution_name: String,
        token_contract: Address,
        base_salary_amount: i128,
        min_promotion_interval: u64,
    );

    /// Add a new employee to the institution
    ///
    /// # Arguments
    ///
    /// * `employee_address` - Address of the new employee
    /// * `name` - Employee's name
    /// * `rank` - Initial rank of the employee
    /// * `department` - Department the employee belongs to
    ///
    /// # Returns
    ///
    /// Employee ID of the newly added employee
    ///
    /// # Events
    ///
    /// Emits an event with topics `["employee_added", employee_id: u64]`,
    /// data = `EmployeeAddedEvent`
    fn add_employee(
        env: Env,
        employee_address: Address,
        name: String,
        rank: EmployeeRank,
        department: String,
    ) -> u64;

    /// Remove an employee from the institution
    ///
    /// # Arguments
    ///
    /// * `employee_id` - ID of the employee to remove
    ///
    /// # Events
    ///
    /// Emits an event with topics `["employee_removed", employee_id: u64]`
    fn remove_employee(env: Env, employee_id: u64);

    /// Update employee information
    ///
    /// # Arguments
    ///
    /// * `employee_id` - ID of the employee to update
    /// * `name` - New name (optional)
    /// * `department` - New department (optional)
    /// * `base_salary` - New base salary (optional)
    fn update_employee(
        env: Env,
        employee_id: u64,
        name: Option<String>,
        department: Option<String>,
        base_salary: Option<i128>,
    );

    /// Promote an employee to the next rank
    ///
    /// # Arguments
    ///
    /// * `employee_id` - ID of the employee to promote
    ///
    /// # Events
    ///
    /// Emits an event with topics `["employee_promoted", employee_id: u64]`,
    /// data = `EmployeePromotedEvent`
    fn promote_employee(env: Env, employee_id: u64);

    /// Suspend an employee
    ///
    /// # Arguments
    ///
    /// * `employee_id` - ID of the employee to suspend
    ///
    /// # Events
    ///
    /// Emits an event with topics `["employee_status_changed", employee_id: u64]`,
    /// data = `EmployeeStatusChangedEvent`
    fn suspend_employee(env: Env, employee_id: u64);

    /// Reactivate a suspended employee
    ///
    /// # Arguments
    ///
    /// * `employee_id` - ID of the employee to reactivate
    ///
    /// # Events
    ///
    /// Emits an event with topics `["employee_status_changed", employee_id: u64]`,
    /// data = `EmployeeStatusChangedEvent`
    fn reactivate_employee(env: Env, employee_id: u64);

    /// Pay salary to an employee using the SEP-41 token
    ///
    /// # Arguments
    ///
    /// * `employee_id` - ID of the employee to pay
    ///
    /// # Events
    ///
    /// Emits an event with topics `["salary_paid", employee_id: u64]`,
    /// data = `SalaryPaidEvent`
    fn pay_salary(env: Env, employee_id: u64);

    /// Pay salary to all active employees
    ///
    /// # Events
    ///
    /// Emits multiple `salary_paid` events for each employee
    fn pay_all_salaries(env: Env);

    /// Get employee information by ID
    ///
    /// # Arguments
    ///
    /// * `employee_id` - ID of the employee
    ///
    /// # Returns
    ///
    /// Employee information
    fn get_employee(env: Env, employee_id: u64) -> Employee;

    /// Get employee ID by address
    ///
    /// # Arguments
    ///
    /// * `employee_address` - Address of the employee
    ///
    /// # Returns
    ///
    /// Employee ID if found
    fn get_employee_by_address(env: Env, employee_address: Address) -> Option<u64>;

    /// Get all employees (paginated)
    ///
    /// # Arguments
    ///
    /// * `start_id` - Starting employee ID for pagination
    /// * `limit` - Maximum number of employees to return
    ///
    /// # Returns
    ///
    /// Vector of employees
    fn get_employees(env: Env, start_id: u64, limit: u32) -> Vec<Employee>;

    /// Get institution information
    ///
    /// # Returns
    ///
    /// Institution information
    fn get_institution(env: Env) -> Institution;

    /// Get total number of employees
    ///
    /// # Returns
    ///
    /// Total employee count
    fn get_employee_count(env: Env) -> u64;

    /// Check if the contract is initialized
    ///
    /// # Returns
    ///
    /// True if initialized, false otherwise
    fn initialized(env: Env) -> bool;
}

/// Administrative interface for the employee management system
pub trait EmployeeManagementAdminInterface {
    /// Set a new admin for the system
    ///
    /// # Arguments
    ///
    /// * `new_admin` - Address of the new admin
    fn set_admin(env: Env, new_admin: Address);

    /// Get the current admin address
    ///
    /// # Returns
    ///
    /// Current admin address
    fn get_admin(env: Env) -> Address;

    /// Update institution information
    ///
    /// # Arguments
    ///
    /// * `name` - New institution name (optional)
    /// * `token_contract` - New token contract address (optional)
    /// * `base_salary_amount` - New base salary amount (optional)
    /// * `min_promotion_interval` - New minimum promotion interval (optional)
    fn update_institution(
        env: Env,
        name: Option<String>,
        token_contract: Option<Address>,
        base_salary_amount: Option<i128>,
        min_promotion_interval: Option<u64>,
    );
}
