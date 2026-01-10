use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, Symbol};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Employee {
    id: String,
    name: String,
    institution: String,
    salary: i128,
    rank: String,
    status: String,
    hire_date: u64,
}

#[contracttype]
pub enum DataKey {
    Employees(String),
    EmployeeCount,
    Admin,
}

#[contract]
pub struct EmployeeManagementContract;

#[contractimpl]
impl EmployeeManagementContract {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage().persistent().set(&DataKey::EmployeeCount, &0u32);
    }

    pub fn add_employee(
        env: Env,
        id: String,
        name: String,
        institution: String,
        salary: i128,
        rank: String,
    ) -> Result<(), soroban_sdk::Error> {
        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        if env.storage().persistent().has(&DataKey::Employees(id.clone())) {
            return Err(env.errors().get(Symbol::new(&env, "EmpExists")).unwrap());
        }

        if salary < 0 {
            return Err(env.errors().get(Symbol::new(&env, "InvSalary")).unwrap());
        }

        let employee = Employee {
            id: id.clone(),
            name,
            institution,
            salary,
            rank,
            status: String::from_str(&env, "Active"),
            hire_date: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&DataKey::Employees(id), &employee);
        let count: u32 = env.storage().persistent().get(&DataKey::EmployeeCount).unwrap_or(0);
        env.storage().persistent().set(&DataKey::EmployeeCount, &(count + 1));
        Ok(())
    }

    pub fn remove_employee(env: Env, id: String) -> Result<(), soroban_sdk::Error> {
        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        if !env.storage().persistent().has(&DataKey::Employees(id.clone())) {
            return Err(env.errors().get(Symbol::new(&env, "NotFound")).unwrap());
        }

        env.storage().persistent().remove(&DataKey::Employees(id));
        let count: u32 = env.storage().persistent().get(&DataKey::EmployeeCount).unwrap();
        env.storage().persistent().set(&DataKey::EmployeeCount, &(count - 1));
        Ok(())
    }

    pub fn update_employee(
        env: Env,
        id: String,
        name: Option<String>,
        salary: Option<i128>,
        institution: Option<String>,
    ) -> Result<(), soroban_sdk::Error> {
        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&DataKey::Employees(id.clone()))
            .ok_or_else(|| env.errors().get(Symbol::new(&env, "NotFound")).unwrap())?;

        if let Some(new_name) = name {
            employee.name = new_name;
        }
        if let Some(new_salary) = salary {
            if new_salary < 0 {
                return Err(env.errors().get(Symbol::new(&env, "InvSalary")).unwrap());
            }
            employee.salary = new_salary;
        }
        if let Some(new_institution) = institution {
            employee.institution = new_institution;
        }

        env.storage().persistent().set(&DataKey::Employees(id), &employee);
        Ok(())
    }

    pub fn promote_employee(env: Env, id: String, new_rank: String) -> Result<(), soroban_sdk::Error> {
        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&DataKey::Employees(id.clone()))
            .ok_or_else(|| env.errors().get(Symbol::new(&env, "NotFound")).unwrap())?;

        if employee.status != String::from_str(&env, "Active") {
            return Err(env.errors().get(Symbol::new(&env, "NotActive")).unwrap());
        }

        employee.rank = new_rank;
        env.storage().persistent().set(&DataKey::Employees(id), &employee);
        Ok(())
    }

    pub fn suspend_employee(env: Env, id: String) -> Result<(), soroban_sdk::Error> {
        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&DataKey::Employees(id.clone()))
            .ok_or_else(|| env.errors().get(Symbol::new(&env, "NotFound")).unwrap())?;

        if employee.status == String::from_str(&env, "Suspended") {
            return Err(env.errors().get(Symbol::new(&env, "AlreadySusp")).unwrap());
        }

        employee.status = String::from_str(&env, "Suspended");
        env.storage().persistent().set(&DataKey::Employees(id), &employee);
        Ok(())
    }

    pub fn get_employee(env: Env, id: String) -> Result<Employee, soroban_sdk::Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Employees(id))
            .ok_or_else(|| env.errors().get(Symbol::new(&env, "NotFound")).unwrap())
    }

    pub fn get_employee_count(env: Env) -> u32 {
        env.storage().persistent().get(&DataKey::EmployeeCount).unwrap_or(0)
    }
}