use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

pub trait TokenInterface {
    fn allowance(env: Env, owner: Address, spender: Address) -> i128;
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);
    fn balance(env: Env, id: Address) -> i128;
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);
    fn burn(env: Env, from: Address, amount: i128);
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);
    fn decimals(env: Env) -> u32;
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
}

#[contracttype]
pub struct TransferEvent {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub amount: i128,
}

#[contracttype]
pub struct ApprovalEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

#[contract]
pub struct Sep41;

#[contractimpl]
impl TokenInterface for Sep41 {
    fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        let (amount, expiration_ledger): (i128, u32) = env
            .storage()
            .persistent()
            .get(&(owner, spender))
            .unwrap_or((0, 0));
        
        if amount > 0 && expiration_ledger > env.ledger().sequence() {
            amount
        } else {
            0
        }
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        
        if amount < 0 {
            panic!("negative amount not allowed");
        }

        env.storage()
            .persistent()
            .set(&(from.clone(), spender.clone()), &(amount, expiration_ledger));

        env.events().publish(
            ("approve", &from, &spender),
            ApprovalEvent {
                from: from.clone(),
                to: spender.clone(),
                amount,
            },
        );
    }

    fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&id).unwrap_or(0)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        if amount < 0 {
            panic!("negative amount not allowed");
        }

        let from_balance = env.storage().persistent().get(&from).unwrap_or(0);
        if from_balance < amount {
            panic!("insufficient balance");
        }

        env.storage().persistent().set(&from, &(from_balance - amount));
        let to_balance = env.storage().persistent().get(&to).unwrap_or(0);
        env.storage().persistent().set(&to, &(to_balance + amount));

        env.events().publish(
            ("transfer", &from, &to),
            TransferEvent {
                from: Some(from.clone()),
                to: Some(to.clone()),
                amount,
            },
        );
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        
        if amount < 0 {
            panic!("negative amount not allowed");
        }

        let (current_allowance, expiration_ledger): (i128, u32) = env
            .storage()
            .persistent()
            .get(&(from.clone(), spender.clone()))
            .unwrap_or((0, 0));

        if current_allowance < amount {
            panic!("insufficient allowance");
        }

        if expiration_ledger <= env.ledger().sequence() {
            panic!("allowance expired");
        }

        Self::transfer(env.clone(), from.clone(), to, amount);

        let new_allowance = current_allowance - amount;
        if new_allowance > 0 {
            env.storage()
                .persistent()
                .set(&(from, spender), &(new_allowance, expiration_ledger));
        } else {
            env.storage().persistent().remove(&(from, spender));
        }
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        
        if amount < 0 {
            panic!("negative amount not allowed");
        }

        let from_balance = env.storage().persistent().get(&from).unwrap_or(0);
        if from_balance < amount {
            panic!("insufficient balance");
        }

        env.storage().persistent().set(&from, &(from_balance - amount));

        env.events().publish(
            ("transfer", &from),
            TransferEvent {
                from: Some(from.clone()),
                to: None,
                amount,
            },
        );
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        if amount < 0 {
            panic!("negative amount not allowed");
        }
        let (current_allowance, expiration_ledger): (i128, u32) = env
            .storage()
            .persistent()
            .get(&(from.clone(), spender.clone()))
            .unwrap_or((0, 0));

        if current_allowance < amount {
            panic!("insufficient allowance");
        }
        if expiration_ledger <= env.ledger().sequence() {
            panic!("allowance expired");
        }
        Self::burn(env.clone(), from.clone(), amount);
        let new_allowance = current_allowance - amount;
        if new_allowance > 0 {
            env.storage()
                .persistent()
                .set(&(from, spender), &(new_allowance, expiration_ledger));
        } else {
            env.storage().persistent().remove(&(from, spender));
        }
    }

    fn decimals(_env: Env) -> u32 {
        18
    }

    fn name(env: Env) -> String {
        String::from_str(&env, "Age Devs")
    }

    fn symbol(env: Env) -> String {
        String::from_str(&env, "AGEDEVS")
    }
}

#[contractimpl]
impl Sep41 {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&"admin") {
            panic!("already initialized");
        }
        env.storage().instance().set(&"admin", &admin);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&"admin")
            .expect("not initialized");
        admin.require_auth();

        if amount < 0 {
            panic!("negative amount not allowed");
        }

        let balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&to, &(balance + amount));

        env.events().publish(
            ("transfer", &to),
            TransferEvent {
                from: None,
                to: Some(to.clone()),
                amount,
            },
        );
    }

    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&"admin")
            .expect("not initialized")
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&"admin")
            .expect("not initialized");
        admin.require_auth();

        env.storage().instance().set(&"admin", &new_admin);
    }
}
