// src/lib.rs
#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env, Symbol, Vec, Map};

type Amount = i128;
type LedgerSeq = u32;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    TokenContract,
    Employees, // Map<Address, Employee>
}

#[contracttype]
#[derive(Clone)]
pub enum Status {
    Active,
    Suspended,
    Removed,
}

#[contracttype]
#[derive(Clone)]
pub struct Employee {
    pub institution: Address,
    pub price: Amount,
    pub rank: u8,
    pub status: Status,
    pub joined_ledger: LedgerSeq,
}

/// Employee management contract
pub struct EmployeeContract;

#[contractimpl]
impl EmployeeContract {
    /// Initialize contract with admin and the SEP-41 token contract address.
    /// This should be called once.
    pub fn initialize(env: Env, admin: Address, token_contract: Address) {
        if env.storage().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().set(&DataKey::Admin, &admin.clone());
        env.storage().set(&DataKey::TokenContract, &token_contract.clone());

        // create empty employees map
        let employees: Map<Address, Employee> = Map::new(&env);
        env.storage().set(&DataKey::Employees, &employees);
    }

    /// Register a new employee for `institution` with agreed `price` and `rank`.
    /// Must be called/signed by the institution.
    pub fn register_employee(env: Env, institution: Address, employee: Address, price: Amount, rank: u8) {
        // ensure institution signed
        institution.require_auth();

        if price <= 0 {
            panic!("price must be positive");
        }

        let ledger_seq: LedgerSeq = env.ledger().sequence().into();

        let mut employees: Map<Address, Employee> = env.storage().get_unchecked(&DataKey::Employees).unwrap();

        if employees.get(employee.clone()).is_some() {
            panic!("employee already exists");
        }

        let e = Employee {
            institution: institution.clone(),
            price,
            rank,
            status: Status::Active,
            joined_ledger: ledger_seq,
        };

        employees.set(employee.clone(), e);
        env.storage().set(&DataKey::Employees, &employees);

        // Optionally emit an event
        env.events().publish(
            (Symbol::short("employee_registered"), institution.clone(), employee.clone()),
            (price, rank, ledger_seq),
        );
    }

    /// Update the agreed price for an employee.
    /// Only the institution that registered the employee can update.
    pub fn update_price(env: Env, institution: Address, employee: Address, new_price: Amount) {
        institution.require_auth();
        if new_price <= 0 {
            panic!("price must be positive");
        }

        let mut employees: Map<Address, Employee> = env.storage().get_unchecked(&DataKey::Employees).unwrap();
        let mut entry = employees.get(employee.clone()).expect("employee not found");

        if entry.institution != institution {
            panic!("only registering institution can update price");
        }

        if matches!(entry.status, Status::Removed) {
            panic!("cannot update removed employee");
        }

        entry.price = new_price;
        employees.set(employee.clone(), entry.clone());
        env.storage().set(&DataKey::Employees, &employees);

        env.events().publish((Symbol::short("employee_price_updated"), institution.clone(), employee.clone()), new_price);
    }

    /// Promote -> increase rank by `by` (u8).
    /// Only the institution who registered the employee can promote.
    pub fn promote(env: Env, institution: Address, employee: Address, by: u8) {
        institution.require_auth();
        let mut employees: Map<Address, Employee> = env.storage().get_unchecked(&DataKey::Employees).unwrap();
        let mut entry = employees.get(employee.clone()).expect("employee not found");

        if entry.institution != institution {
            panic!("only registering institution can promote");
        }
        if matches!(entry.status, Status::Removed) {
            panic!("cannot promote removed employee");
        }

        // handle overflow defensively
        let new_rank = entry.rank.saturating_add(by);
        entry.rank = new_rank;
        employees.set(employee.clone(), entry.clone());
        env.storage().set(&DataKey::Employees, &employees);

        env.events().publish((Symbol::short("employee_promoted"), institution.clone(), employee.clone()), new_rank);
    }

    /// Suspend an employee (only institution can suspend)
    pub fn suspend(env: Env, institution: Address, employee: Address) {
        institution.require_auth();
        let mut employees: Map<Address, Employee> = env.storage().get_unchecked(&DataKey::Employees).unwrap();
        let mut entry = employees.get(employee.clone()).expect("employee not found");

        if entry.institution != institution {
            panic!("only registering institution can suspend");
        }
        if matches!(entry.status, Status::Removed) {
            panic!("cannot suspend removed employee");
        }

        entry.status = Status::Suspended;
        employees.set(employee.clone(), entry.clone());
        env.storage().set(&DataKey::Employees, &employees);

        env.events().publish((Symbol::short("employee_suspended"), institution.clone(), employee.clone()), ());
    }

    /// Unsuspend (reactivate) an employee
    pub fn unsuspend(env: Env, institution: Address, employee: Address) {
        institution.require_auth();
        let mut employees: Map<Address, Employee> = env.storage().get_unchecked(&DataKey::Employees).unwrap();
        let mut entry = employees.get(employee.clone()).expect("employee not found");

        if entry.institution != institution {
            panic!("only registering institution can unsuspend");
        }
        if matches!(entry.status, Status::Removed) {
            panic!("cannot unsuspend removed employee");
        }

        entry.status = Status::Active;
        employees.set(employee.clone(), entry.clone());
        env.storage().set(&DataKey::Employees, &employees);

        env.events().publish((Symbol::short("employee_unsuspended"), institution.clone(), employee.clone()), ());
    }

    /// Remove employee (mark as removed). Can be called by the original institution OR admin.
    pub fn remove_employee(env: Env, caller: Address, employee: Address) {
        // caller must be either admin or the institution that registered employee.
        // first check admin
        let admin: Address = env.storage().get_unchecked(&DataKey::Admin).expect("not initialized");
        let mut employees: Map<Address, Employee> = env.storage().get_unchecked(&DataKey::Employees).unwrap();
        let mut entry = employees.get(employee.clone()).expect("employee not found");

        // require caller auth
        caller.require_auth();

        let is_admin = caller == admin;
        let is_institution = caller == entry.institution;

        if !is_admin && !is_institution {
            panic!("only admin or registering institution can remove employee");
        }

        entry.status = Status::Removed;
        employees.set(employee.clone(), entry.clone());
        env.storage().set(&DataKey::Employees, &employees);

        env.events().publish((Symbol::short("employee_removed"), caller.clone(), employee.clone()), ());
    }

    /// Get employee details (view)
    pub fn get_employee(env: Env, employee: Address) -> Employee {
        let employees: Map<Address, Employee> = env.storage().get_unchecked(&DataKey::Employees).unwrap();
        employees.get(employee).expect("employee not found")
    }

    /// (Optional) Pay employee: instructs contract to transfer `price` from institution to employee using token.
    /// IMPORTANT: cross-contract call; token must allow this contract to move funds (either via allowance/transfer_from or by signing).
    ///
    /// This method demonstrates how you would invoke the SEP-41 token's transfer_from / transfer function.
    /// The exact function name and call shape might require tiny change depending on your soroban-sdk version.
    pub fn pay_employee(env: Env, institution: Address, employee: Address) {
        institution.require_auth();

        // read token address
        let token_contract: Address = env.storage().get_unchecked(&DataKey::TokenContract).expect("token not set");
        let employees: Map<Address, Employee> = env.storage().get_unchecked(&DataKey::Employees).unwrap();
        let entry = employees.get(employee.clone()).expect("employee not found");

        if entry.institution != institution {
            panic!("only registering institution can pay this employee");
        }
        if matches!(entry.status, Status::Removed) {
            panic!("cannot pay removed employee");
        }
        if matches!(entry.status, Status::Suspended) {
            panic!("cannot pay suspended employee");
        }

        let price: Amount = entry.price;

        // Example cross-contract invocation (pseudocode). The shape below often works:
        // env.invoke_contract(&token_contract, &Symbol::short("transfer_from"), (&Address::from_current_contract(&env), institution.clone(), employee.clone(), price));
        //
        // OR use transfer if the institution signed this contract to move funds on its behalf:
        // env.invoke_contract(&token_contract, &Symbol::short("transfer"), (institution.clone(), employee.clone(), price));
        //
        // The exact invocation and argument order must match the token contract's exported method signature.
        //
        // We don't `panic!` here — replace the two lines above with the exact call for your token and handle result/err.
        //
        // For now, we'll emit an event so the caller/test can see intent:
        env.events().publish((Symbol::short("employee_payment_intent"), institution.clone(), employee.clone()), price);
    }
}
