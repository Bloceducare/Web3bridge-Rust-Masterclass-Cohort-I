#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Symbol, symbol_short
};

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

const ALLOWANCE: Symbol = symbol_short!("ALLOWANCE");
const BALANCE: Symbol = symbol_short!("BALANCE");
const NONCE: Symbol = symbol_short!("NONCE");
const STATE: Symbol = symbol_short!("STATE");
const ADMIN: Symbol = symbol_short!("ADMIN");
const METADATA: Symbol = symbol_short!("METADATA");

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized")
        }

        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&METADATA, &TokenMetadata {
            name,
            symbol,
            decimals,
        });
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        env.storage().instance().extend_ttl(1000, 1000);
        receive_balance(&env, to.clone(), amount);
        TokenUtils::new(&env).events().mint(admin, to, amount);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = AllowanceDataKey { from, spender };
        let allowance = env.storage().temporary().get::<AllowanceDataKey, AllowanceValue>(&key);
        if let Some(allowance) = allowance {
            if allowance.expiration_ledger < env.ledger().sequence() {
                0
            } else {
                allowance.amount
            }
        } else {
            0
        }
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        let ledger = env.ledger().sequence();

        if expiration_ledger < ledger {
            panic!("expiration_ledger is less than ledger")
        }

        let key = AllowanceDataKey {
            from: from.clone(),
            spender: spender.clone(),
        };
        let allowance = AllowanceValue {
            amount,
            expiration_ledger,
        };

        env.storage().temporary().set(&key, &allowance);
        env.storage()
            .temporary()
            .extend_ttl(&key, expiration_ledger - ledger, expiration_ledger - ledger);

        TokenUtils::new(&env)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&(BALANCE, id.clone()))
            .unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        spend_balance(&env, from.clone(), amount);
        receive_balance(&env, to.clone(), amount);
        TokenUtils::new(&env).events().transfer(from, to, amount);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        spend_allowance(&env, from.clone(), spender, amount);
        spend_balance(&env, from.clone(), amount);
        receive_balance(&env, to.clone(), amount);
        TokenUtils::new(&env).events().transfer(from, to, amount)
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        spend_balance(&env, from.clone(), amount);
        TokenUtils::new(&env).events().burn(from, amount);
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        spend_allowance(&env, from.clone(), spender, amount);
        spend_balance(&env, from.clone(), amount);
        TokenUtils::new(&env).events().burn(from, amount);
    }

    pub fn decimals(env: Env) -> u32 {
        let metadata: TokenMetadata = env.storage().instance().get(&METADATA).unwrap();
        metadata.decimals
    }

    pub fn name(env: Env) -> String {
        let metadata: TokenMetadata = env.storage().instance().get(&METADATA).unwrap();
        metadata.name
    }

    pub fn symbol(env: Env) -> String {
        let metadata: TokenMetadata = env.storage().instance().get(&METADATA).unwrap();
        metadata.symbol
    }
}

fn receive_balance(env: &Env, addr: Address, amount: i128) {
    let balance = read_balance(env, addr.clone());
    env.storage()
        .persistent()
        .set(&(BALANCE, addr), &(balance + amount));
}

fn spend_balance(env: &Env, addr: Address, amount: i128) {
    let balance = read_balance(env, addr.clone());
    if balance < amount {
        panic!("insufficient balance");
    }
    env.storage()
        .persistent()
        .set(&(BALANCE, addr), &(balance - amount));
}

fn read_balance(env: &Env, addr: Address) -> i128 {
    let key = (BALANCE, addr);
    env.storage().persistent().get(&key).unwrap_or(0)
}

fn spend_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = read_allowance(env, from.clone(), spender.clone());
    if allowance < amount {
        panic!("insufficient allowance");
    }
    approve_allowance(
        env,
        from,
        spender,
        allowance - amount,
        env.ledger().sequence() + 100,
    );
}

fn read_allowance(env: &Env, from: Address, spender: Address) -> i128 {
    let key = AllowanceDataKey { from, spender };
    let allowance = env.storage().temporary().get::<AllowanceDataKey, AllowanceValue>(&key);
    if let Some(allowance) = allowance {
        if allowance.expiration_ledger < env.ledger().sequence() {
            0
        } else {
            allowance.amount
        }
    } else {
        0
    }
}

fn approve_allowance(
    env: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let key = AllowanceDataKey { from, spender };
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    env.storage().temporary().set(&key, &allowance);
    let ledger = env.ledger().sequence();
    env.storage()
        .temporary()
        .extend_ttl(&key, expiration_ledger - ledger, expiration_ledger - ledger);
}

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {
    InternalError = 1,
    AlreadyInitializedError = 3,
    UnauthorizedError = 4,
    NegativeAmountError = 8,
    AllowanceError = 9,
    BalanceError = 10,
    OverflowError = 12,
}

pub struct TokenUtils {
    env: Env,
}

impl TokenUtils {
    fn new(env: &Env) -> TokenUtils {
        TokenUtils { env: env.clone() }
    }

    fn events(&self) -> TokenEvents {
        TokenEvents::new(&self.env)
    }
}

pub struct TokenEvents {
    env: Env,
}

impl TokenEvents {
    fn new(env: &Env) -> TokenEvents {
        TokenEvents { env: env.clone() }
    }

    fn mint(&self, admin: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("mint"), admin, to);
        self.env.events().publish(topics, amount);
    }

    fn transfer(&self, from: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("transfer"), from, to);
        self.env.events().publish(topics, amount);
    }

    fn burn(&self, from: Address, amount: i128) {
        let topics = (symbol_short!("burn"), from);
        self.env.events().publish(topics, amount);
    }

    fn approve(&self, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        let topics = (symbol_short!("approve"), from, spender);
        self.env.events().publish(topics, (amount, expiration_ledger));
    }
}

#[cfg(test)]
mod test;