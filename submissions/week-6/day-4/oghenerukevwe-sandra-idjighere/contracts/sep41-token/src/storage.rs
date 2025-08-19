use soroban_sdk::{contracttype, Address, Env, String};

/// Storage keys for the token contract
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Token balance for an address
    Balance(Address),
    /// Allowance from one address to another with expiration
    Allowance(AllowanceDataKey),
    /// Token metadata
    TokenInfo,
    /// Contract admin
    Admin,
    /// Initialization status
    Initialized,
}

/// Allowance storage key structure
#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

/// Allowance value with expiration
#[derive(Clone)]
#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

/// Token metadata information
#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

/// Storage operations for token balances
pub fn read_balance(env: &Env, addr: &Address) -> i128 {
    let key = DataKey::Balance(addr.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn write_balance(env: &Env, addr: &Address, amount: i128) {
    let key = DataKey::Balance(addr.clone());
    env.storage().persistent().set(&key, &amount);
}

/// Storage operations for allowances
pub fn read_allowance(env: &Env, from: &Address, spender: &Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });
    
    if let Some(allowance) = env.storage().temporary().get::<DataKey, AllowanceValue>(&key) {
        // Check if allowance has expired
        if allowance.expiration_ledger < env.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                expiration_ledger: allowance.expiration_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}

pub fn write_allowance(
    env: &Env,
    from: &Address,
    spender: &Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let key = DataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });
    
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };
    
    // Calculate TTL for temporary storage
    let current_ledger = env.ledger().sequence();
    let ttl = if expiration_ledger > current_ledger {
        expiration_ledger - current_ledger
    } else {
        1 // Minimum TTL
    };
    
    env.storage().temporary().set(&key, &allowance);
    env.storage().temporary().extend_ttl(&key, ttl, ttl);
}

/// Storage operations for token metadata
pub fn read_token_info(env: &Env) -> TokenMetadata {
    let key = DataKey::TokenInfo;
    env.storage().instance().get(&key).unwrap()
}

pub fn write_token_info(env: &Env, metadata: &TokenMetadata) {
    let key = DataKey::TokenInfo;
    env.storage().instance().set(&key, metadata);
}

/// Storage operations for admin
pub fn read_admin(env: &Env) -> Address {
    let key = DataKey::Admin;
    env.storage().instance().get(&key).unwrap()
}

pub fn write_admin(env: &Env, admin: &Address) {
    let key = DataKey::Admin;
    env.storage().instance().set(&key, admin);
}

/// Storage operations for initialization status
pub fn is_initialized(env: &Env) -> bool {
    let key = DataKey::Initialized;
    env.storage().instance().get(&key).unwrap_or(false)
}

pub fn set_initialized(env: &Env) {
    let key = DataKey::Initialized;
    env.storage().instance().set(&key, &true);
}

/// Helper function to spend allowance
pub fn spend_allowance(env: &Env, from: &Address, spender: &Address, amount: i128) {
    let allowance = read_allowance(env, from, spender);
    
    if allowance.amount < amount {
        panic!("insufficient allowance");
    }
    
    write_allowance(
        env,
        from,
        spender,
        allowance.amount - amount,
        allowance.expiration_ledger,
    );
}

/// Helper function to check and update balance
pub fn transfer_balance(env: &Env, from: &Address, to: &Address, amount: i128) {
    let from_balance = read_balance(env, from);
    
    if from_balance < amount {
        panic!("insufficient balance");
    }
    
    write_balance(env, from, from_balance - amount);
    
    let to_balance = read_balance(env, to);
    write_balance(env, to, to_balance + amount);
}
