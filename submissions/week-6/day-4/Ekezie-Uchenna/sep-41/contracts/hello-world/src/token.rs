use soroban_sdk::{ contract, contractimpl, Address, Env, String };

use crate::storage::{ AllowanceKey, AllowanceValue, DataKey };
use crate::traits::TokenInterface;

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn __constructor(env: Env, admin: Address, name: String, symbol: String, decimals: u32) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Token contract already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);

        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin = env.storage().instance().get::<DataKey, Address>(&DataKey::Admin).unwrap();

        admin.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let to_key = DataKey::Balance(to.clone());
        let to_balance = Self::balance(env.clone(), to);

        env.storage()
            .persistent()
            .set(&to_key, &(to_balance + amount));

        let total_supply = env
            .storage()
            .instance()
            .get::<DataKey, i128>(&DataKey::TotalSupply)
            .unwrap();

        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(total_supply + amount));
    }

    pub fn get_total_supply(env: Env) -> i128 {
        let total_supply = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        total_supply
    }
}

#[contractimpl]
impl TokenInterface for TokenContract {
    fn name(env: Env) -> String {
        let name = env.storage().instance().get(&DataKey::Name).unwrap();
        name
    }
    fn symbol(env: Env) -> String {
        let symbol = env.storage().instance().get(&DataKey::Symbol).unwrap();
        symbol
    }
    fn decimals(env: Env) -> u32 {
        let decimals = env.storage().instance().get(&DataKey::Decimals).unwrap();
        decimals
    }

    fn balance(env: Env, id: Address) -> i128 {
        let key = DataKey::Balance(id);
        let balance = env.storage().persistent().get(&key).unwrap_or(0);
        balance
    }
    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }

        let from_key = DataKey::Balance(from.clone());
        let from_balance = Self::balance(env.clone(), from);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        env.storage()
            .persistent()
            .set(&from_key, &(from_balance - amount));

        let to_key = DataKey::Balance(to.clone());
        let to_balance = Self::balance(env.clone(), to);

        env.storage()
            .persistent()
            .set(&to_key, &(to_balance + amount));
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        let key = DataKey::Allowance(AllowanceKey { from, spender });
        let allowance = AllowanceValue {
            amount,
            expiration_ledger,
        };

        env.storage().persistent().set(&key, &allowance);
    }

    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = DataKey::Allowance(AllowanceKey { from, spender });

        match env.storage().persistent().get::<DataKey, AllowanceValue>(&key) {
            Some(allowance) => {
                if env.ledger().sequence() > allowance.expiration_ledger {
                    0
                } else {
                    allowance.amount
                }
            }
            None => 0,
        }
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }

        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        let allowance = Self::allowance(env.clone(), from.clone(), spender);

        if amount > allowance {
            panic!("Insufficient allowance");
        }

        let current_allowance: AllowanceValue = env
            .storage()
            .persistent()
            .get(&allowance_key)
            .unwrap();
        let new_allowance = AllowanceValue {
            amount: current_allowance.amount - amount,
            expiration_ledger: current_allowance.expiration_ledger,
        };
        env.storage().persistent().set(&allowance_key, &new_allowance);

        Self::transfer(env, from, to, amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        let key = DataKey::Balance(from.clone());
        let balance = Self::balance(env.clone(), from);

        if amount > balance {
            panic!("Insufficient balance");
        }

        let total_supply = Self::get_total_supply(env.clone());
        let new_total_supply = total_supply - amount;

        env.storage()
            .persistent()
            .set(&key, &(balance - amount));

        env.storage().instance().set(&DataKey::TotalSupply, &new_total_supply);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        if amount <= 0 {
            panic!("Amount must be greater than zero");
        }

        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        let allowance = Self::allowance(env.clone(), from.clone(), spender);

        if amount > allowance {
            panic!("Insufficient allowance");
        }

        let current_allowance: AllowanceValue = env
            .storage()
            .persistent()
            .get(&allowance_key)
            .unwrap();
        let new_allowance = AllowanceValue {
            amount: current_allowance.amount - amount,
            expiration_ledger: current_allowance.expiration_ledger,
        };

        env.storage().persistent().set(&allowance_key, &new_allowance);

        let from_key = DataKey::Balance(from.clone());
        let from_balance = Self::balance(env.clone(), from);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let new_balance = from_balance - amount;
        env.storage().persistent().set(&from_key, &new_balance);

        let total_supply = Self::get_total_supply(env.clone());
        let new_total_supply = total_supply - amount;

        env.storage().instance().set(&DataKey::TotalSupply, &new_total_supply);
    }
}
