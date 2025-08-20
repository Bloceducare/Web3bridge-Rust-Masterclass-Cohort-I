#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contract]
pub struct TokenContract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
    Allowance(Address, Address),
    Name,
    Symbol,
    Decimals,
    TotalSupply,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowanceValue {
    pub amount: i128,
    pub live_until_ledger: u32,
}

#[contractimpl]
impl TokenContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) {
        admin.require_auth();
        
        if env.storage().instance().has(&DataKey::Name) {
            panic!("Already initialized");
        }
        
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }
    
    pub fn name(env: Env) -> String {
        env.storage().instance().get(&DataKey::Name)
            .unwrap_or_else(|| panic!("Not initialized"))
    }
    
    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol)
            .unwrap_or_else(|| panic!("Not initialized"))
    }
    
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimals)
            .unwrap_or_else(|| panic!("Not initialized"))
    }
    
    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(id)).unwrap_or(0)
    }
    
    fn get_balance(env: &Env, addr: &Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(addr.clone())).unwrap_or(0)
    }
    
    fn set_balance(env: &Env, addr: &Address, amount: i128) {
        if amount > 0 {
            env.storage().persistent().set(&DataKey::Balance(addr.clone()), &amount);
        } else {
            env.storage().persistent().remove(&DataKey::Balance(addr.clone()));
        }
    }
    
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        
        if amount <= 0 {
            panic!("Amount must be positive");
        }
        
        let from_balance = Self::get_balance(&env, &from);
        if from_balance < amount {
            panic!("Insufficient balance");
        }
        
        Self::set_balance(&env, &from, from_balance - amount);
        Self::set_balance(&env, &to, Self::get_balance(&env, &to) + amount);
        
        env.events().publish(("transfer",), (from.clone(), to.clone(), amount));
    }
    
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        
        if amount <= 0 {
            panic!("Amount must be positive");
        }
        
        let allowance_key = DataKey::Allowance(from.clone(), spender.clone());
        let allowance_data: AllowanceValue = env.storage().persistent()
            .get(&allowance_key)
            .unwrap_or(AllowanceValue { amount: 0, live_until_ledger: 0 });
            
        if allowance_data.amount < amount {
            panic!("Insufficient allowance");
        }
        
        let from_balance = Self::get_balance(&env, &from);
        if from_balance < amount {
            panic!("Insufficient balance");
        }
        
        Self::set_balance(&env, &from, from_balance - amount);
        Self::set_balance(&env, &to, Self::get_balance(&env, &to) + amount);
        
        let new_allowance = AllowanceValue {
            amount: allowance_data.amount - amount,
            live_until_ledger: allowance_data.live_until_ledger,
        };
        
        if new_allowance.amount > 0 {
            env.storage().persistent().set(&allowance_key, &new_allowance);
        } else {
            env.storage().persistent().remove(&allowance_key);
        }
        
        env.events().publish(("transfer",), (from.clone(), to.clone(), amount));
    }
    
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        from.require_auth();
        
        if amount < 0 {
            panic!("Amount cannot be negative");
        }
        
        let allowance_data = AllowanceValue {
            amount,
            live_until_ledger,
        };
        
        let allowance_key = DataKey::Allowance(from.clone(), spender.clone());
        
        if amount > 0 {
            env.storage().persistent().set(&allowance_key, &allowance_data);
        } else {
            env.storage().persistent().remove(&allowance_key);
        }
        
        env.events().publish(("approve",), (from.clone(), spender.clone(), amount));
    }
    
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let allowance_key = DataKey::Allowance(from, spender);
        let allowance_data: AllowanceValue = env.storage().persistent()
            .get(&allowance_key)
            .unwrap_or(AllowanceValue { amount: 0, live_until_ledger: 0 });
            
        allowance_data.amount
    }
    
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        
        if amount <= 0 {
            panic!("Amount must be positive");
        }
        
        let from_balance = Self::get_balance(&env, &from);
        if from_balance < amount {
            panic!("Insufficient balance");
        }
        
        Self::set_balance(&env, &from, from_balance - amount);
        
        env.events().publish(("burn",), (from.clone(), amount));
    }
    
    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        
        if amount <= 0 {
            panic!("Amount must be positive");
        }
        
        let allowance_key = DataKey::Allowance(from.clone(), spender.clone());
        let allowance_data: AllowanceValue = env.storage().persistent()
            .get(&allowance_key)
            .unwrap_or(AllowanceValue { amount: 0, live_until_ledger: 0 });
            
        if allowance_data.amount < amount {
            panic!("Insufficient allowance");
        }
        
        let from_balance = Self::get_balance(&env, &from);
        if from_balance < amount {
            panic!("Insufficient balance");
        }
        
        Self::set_balance(&env, &from, from_balance - amount);
        
        let new_allowance = AllowanceValue {
            amount: allowance_data.amount - amount,
            live_until_ledger: allowance_data.live_until_ledger,
        };
        
        if new_allowance.amount > 0 {
            env.storage().persistent().set(&allowance_key, &new_allowance);
        } else {
            env.storage().persistent().remove(&allowance_key);
        }
        
        env.events().publish(("burn",), (from.clone(), amount));
    }
}

mod test;
