// src/lib.rs
#![no_std]

use soroban_sdk::{
    contractimpl, contracttype, symbol::Symbol, Address, Env, Map, Vec, SymbolShort, IntoVal,
};

type Amount = i128;
type LedgerSeq = u32;

/// Keys used in contract storage
#[contracttype]
pub enum DataKey {
    Balances,
    Allowances, // Map<(Address, Address) -> (Amount, LedgerSeq)>
    TotalSupply,
    Admin,
    Name,
    Symbol,
    Decimals,
}

/// Helper struct stored as allowance value
#[contracttype]
#[derive(Clone)]
pub struct AllowanceEntry {
    pub amount: Amount,
    pub live_until: LedgerSeq,
}

/// The token contract
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    // ========== Initialization ==========
    /// Initialize token metadata and optionally mint initial supply to `admin`.
    /// Only callable once in typical deployments (you could protect via checking Admin is empty).
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
        initial_mint: Amount,
    ) {
        // Require admin auth in many patterns; here we simply set admin.
        // Guard: only allow if Admin not already set
        if env.storage().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        env.storage().set(&DataKey::Admin, &admin.clone());

        env.storage().set(&DataKey::Name, &name.clone());
        env.storage().set(&DataKey::Symbol, &symbol.clone());
        env.storage().set(&DataKey::Decimals, &decimals);

        // set empty balances/allowances maps
        let balances: Map<Address, Amount> = Map::new(&env);
        let allowances: Map<(Address, Address), AllowanceEntry> = Map::new(&env);

        env.storage().set(&DataKey::Balances, &balances);
        env.storage().set(&DataKey::Allowances, &allowances);

        env.storage().set(&DataKey::TotalSupply, &0i128);

        if initial_mint > 0 {
            // mint to admin
            Self::mint(env.clone(), admin, initial_mint);
        }
    }

    // ========== Metadata ==========
    pub fn name(env: Env) -> String {
        env.storage()
            .get_unchecked(&DataKey::Name)
            .unwrap_or_else(|| String::from(""))
    }

    pub fn symbol(env: Env) -> String {
        env.storage()
            .get_unchecked(&DataKey::Symbol)
            .unwrap_or_else(|| String::from(""))
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage()
            .get_unchecked(&DataKey::Decimals)
            .unwrap_or_else(|| 0u32)
    }

    // ========== Views ==========
    pub fn total_supply(env: Env) -> Amount {
        env.storage().get_unchecked(&DataKey::TotalSupply).unwrap_or(0i128)
    }

    pub fn balance(env: Env, id: Address) -> Amount {
        let balances: Map<Address, Amount> = env.storage().get_unchecked(&DataKey::Balances).unwrap();
        balances.get(id).unwrap_or(0i128)
    }

    pub fn allowance(env: Env, owner: Address, spender: Address) -> Amount {
        let allowances: Map<(Address, Address), AllowanceEntry> =
            env.storage().get_unchecked(&DataKey::Allowances).unwrap();
        if let Some(entry) = allowances.get((owner.clone(), spender.clone())) {
            // Check expiry - expired => treated as 0
            let ledger_seq: LedgerSeq = env.ledger().sequence().into();
            if entry.live_until < ledger_seq {
                0i128
            } else {
                entry.amount
            }
        } else {
            0i128
        }
    }

    // ========== Allowance management ==========
    /// Approve `spender` to spend `amount` from `from`, valid until `live_until_ledger`.
    pub fn approve(env: Env, from: Address, spender: Address, amount: Amount, live_until_ledger: LedgerSeq) {
        // require auth from the `from` address
        from.require_auth();

        if amount < 0 {
            panic!("amount cannot be negative");
        }

        let current_ledger: LedgerSeq = env.ledger().sequence().into();
        if live_until_ledger < current_ledger && amount != 0 {
            panic!("live_until_ledger cannot be less than current ledger unless amount == 0");
        }

        // mutate allowances map
        let mut allowances: Map<(Address, Address), AllowanceEntry> =
            env.storage().get_unchecked(&DataKey::Allowances).unwrap();

        if amount == 0 {
            // setting to zero -> remove entry if exists
            allowances.remove((from.clone(), spender.clone()));
        } else {
            allowances.set((from.clone(), spender.clone()), AllowanceEntry { amount, live_until: live_until_ledger });
        }

        env.storage().set(&DataKey::Allowances, &allowances);

        // Emit approve event: topics ["approve", from, spender], data [amount, live_until_ledger]
        env.events().publish(
            (Symbol::short("approve"), from.clone(), spender.clone()),
            (amount, live_until_ledger),
        );
    }

    // ========== Transfers ==========
    pub fn transfer(env: Env, from: Address, to: Address, amount: Amount) {
        from.require_auth();
        if amount <= 0 {
            panic!("transfer amount must be > 0");
        }

        // balances
        let mut balances: Map<Address, Amount> = env.storage().get_unchecked(&DataKey::Balances).unwrap();
        let from_balance = balances.get(from.clone()).unwrap_or(0i128);
        if from_balance < amount {
            panic!("insufficient balance");
        }
        let to_balance = balances.get(to.clone()).unwrap_or(0i128);

        balances.set(from.clone(), from_balance - amount);
        balances.set(to.clone(), to_balance + amount);

        env.storage().set(&DataKey::Balances, &balances);

        env.events().publish((Symbol::short("transfer"), from.clone(), to.clone()), amount);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: Amount) {
        spender.require_auth();
        if amount <= 0 {
            panic!("transfer amount must be > 0");
        }

        // check allowance
        let mut allowances: Map<(Address, Address), AllowanceEntry> =
            env.storage().get_unchecked(&DataKey::Allowances).unwrap();

        let entry = allowances.get((from.clone(), spender.clone())).unwrap_or(AllowanceEntry { amount: 0i128, live_until: 0u32 });
        let ledger_seq: LedgerSeq = env.ledger().sequence().into();
        let allowed_amount = if entry.live_until < ledger_seq { 0i128 } else { entry.amount };

        if allowed_amount < amount {
            panic!("insufficient allowance");
        }

        // move balances
        let mut balances: Map<Address, Amount> = env.storage().get_unchecked(&DataKey::Balances).unwrap();
        let from_balance = balances.get(from.clone()).unwrap_or(0i128);
        if from_balance < amount {
            panic!("insufficient balance");
        }
        let to_balance = balances.get(to.clone()).unwrap_or(0i128);

        balances.set(from.clone(), from_balance - amount);
        balances.set(to.clone(), to_balance + amount);
        env.storage().set(&DataKey::Balances, &balances);

        // decrease allowance
        let new_allow = allowed_amount - amount;
        if new_allow == 0 {
            allowances.remove((from.clone(), spender.clone()));
        } else {
            allowances.set((from.clone(), spender.clone()), AllowanceEntry { amount: new_allow, live_until: entry.live_until });
        }
        env.storage().set(&DataKey::Allowances, &allowances);

        env.events().publish((Symbol::short("transfer"), from.clone(), to.clone()), amount);
    }

    // ========== Burn / BurnFrom ==========
    pub fn burn(env: Env, from: Address, amount: Amount) {
        from.require_auth();
        if amount <= 0 {
            panic!("burn amount must be > 0");
        }
        let mut balances: Map<Address, Amount> = env.storage().get_unchecked(&DataKey::Balances).unwrap();
        let from_balance = balances.get(from.clone()).unwrap_or(0i128);
        if from_balance < amount {
            panic!("insufficient balance to burn");
        }

        balances.set(from.clone(), from_balance - amount);
        env.storage().set(&DataKey::Balances, &balances);

        // decrease total supply
        let mut total: Amount = env.storage().get_unchecked(&DataKey::TotalSupply).unwrap_or(0i128);
        total = total - amount;
        env.storage().set(&DataKey::TotalSupply, &total);

        env.events().publish((Symbol::short("burn"), from.clone()), amount);
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: Amount) {
        spender.require_auth();
        if amount <= 0 {
            panic!("burn amount must be > 0");
        }

        // allowance check
        let mut allowances: Map<(Address, Address), AllowanceEntry> =
            env.storage().get_unchecked(&DataKey::Allowances).unwrap();
        let entry = allowances.get((from.clone(), spender.clone())).unwrap_or(AllowanceEntry { amount: 0i128, live_until: 0u32});
        let ledger_seq: LedgerSeq = env.ledger().sequence().into();
        let allowed_amount = if entry.live_until < ledger_seq { 0i128 } else { entry.amount };
        if allowed_amount < amount {
            panic!("insufficient allowance");
        }

        // burn from balance
        let mut balances: Map<Address, Amount> = env.storage().get_unchecked(&DataKey::Balances).unwrap();
        let from_balance = balances.get(from.clone()).unwrap_or(0i128);
        if from_balance < amount {
            panic!("insufficient balance to burn");
        }

        balances.set(from.clone(), from_balance - amount);
        env.storage().set(&DataKey::Balances, &balances);

        // decrease allowance
        let new_allow = allowed_amount - amount;
        if new_allow == 0 {
            allowances.remove((from.clone(), spender.clone()));
        } else {
            allowances.set((from.clone(), spender.clone()), AllowanceEntry { amount: new_allow, live_until: entry.live_until });
        }
        env.storage().set(&DataKey::Allowances, &allowances);

        // decrease total supply
        let mut total: Amount = env.storage().get_unchecked(&DataKey::TotalSupply).unwrap_or(0i128);
        total = total - amount;
        env.storage().set(&DataKey::TotalSupply, &total);

        env.events().publish((Symbol::short("burn"), from.clone()), amount);
    }

    // ========== Admin-only: mint & clawback ==========
    pub fn mint(env: Env, to: Address, amount: Amount) {
        // require admin
        let admin: Address = env.storage().get_unchecked(&DataKey::Admin).unwrap();
        admin.require_auth();

        if amount <= 0 {
            panic!("mint amount must be > 0");
        }

        let mut balances: Map<Address, Amount> = env.storage().get_unchecked(&DataKey::Balances).unwrap();
        let to_balance = balances.get(to.clone()).unwrap_or(0i128);
        balances.set(to.clone(), to_balance + amount);
        env.storage().set(&DataKey::Balances, &balances);

        // increase total supply
        let mut total: Amount = env.storage().get_unchecked(&DataKey::TotalSupply).unwrap_or(0i128);
        total = total + amount;
        env.storage().set(&DataKey::TotalSupply, &total);

        env.events().publish((Symbol::short("mint"), to.clone()), amount);
    }

    pub fn clawback(env: Env, admin_caller: Address, from: Address, amount: Amount) {
        // require admin
        let admin: Address = env.storage().get_unchecked(&DataKey::Admin).unwrap();
        admin.require_auth();
        // (We ignore admin_caller param; admin must be the signer)

        if amount <= 0 {
            panic!("clawback amount must be > 0");
        }

        let mut balances: Map<Address, Amount> = env.storage().get_unchecked(&DataKey::Balances).unwrap();
        let from_balance = balances.get(from.clone()).unwrap_or(0i128);
        if from_balance < amount {
            panic!("insufficient balance to clawback");
        }
        balances.set(from.clone(), from_balance - amount);
        env.storage().set(&DataKey::Balances, &balances);

        // reduce supply or send to admin; here we reduce total supply to reflect clawback removal
        let mut total: Amount = env.storage().get_unchecked(&DataKey::TotalSupply).unwrap_or(0i128);
        total = total - amount;
        env.storage().set(&DataKey::TotalSupply, &total);

        env.events().publish((Symbol::short("clawback"), from.clone()), amount);
    }
}

// ====== Unit tests ======
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Ledger, Env, Address, IntoVal};

    fn setup_env() -> Env {
        let env = Env::default();
        // set ledger sequence and timestamp
        env.ledger().set(Ledger {
            sequence: 1,
            timestamp: 0,
            protocol_version: 1,
        });
        env
    }

    #[test]
    fn test_initialize_and_mint() {
        let env = setup_env();
        let admin = Address::random(&env);
        // initialize with initial mint 1000
        TokenContract::initialize(env.clone(), admin.clone(), "My Token".to_string(), "MTK".to_string(), 6u32, 1000i128);
        let total = TokenContract::total_supply(env.clone());
        assert_eq!(total, 1000i128);

        let bal = TokenContract::balance(env.clone(), admin.clone());
        assert_eq!(bal, 1000i128);
    }

    #[test]
    fn test_transfer_and_allowance_flow() {
        let env = setup_env();
        let admin = Address::random(&env);
        let alice = Address::random(&env);
        let bob = Address::random(&env);

        // init and mint 100 to alice
        TokenContract::initialize(env.clone(), admin.clone(), "T".to_string(), "T".to_string(), 2u32, 0i128);

        // mint to alice by admin
        // simulate admin signer:
        env.set_signer(&admin);
        TokenContract::mint(env.clone(), alice.clone(), 100i128);

        assert_eq!(TokenContract::balance(env.clone(), alice.clone()), 100i128);

        // alice approves bob for 40 until ledger 10
        env.set_signer(&alice);
        TokenContract::approve(env.clone(), alice.clone(), bob.clone(), 40i128, 10u32);

        assert_eq!(TokenContract::allowance(env.clone(), alice.clone(), bob.clone()), 40i128);

        // bob transfers 30 from alice to himself
        env.set_signer(&bob);
        TokenContract::transfer_from(env.clone(), bob.clone(), alice.clone(), bob.clone(), 30i128);

        assert_eq!(TokenContract::balance(env.clone(), alice.clone()), 70i128);
        assert_eq!(TokenContract::balance(env.clone(), bob.clone()), 30i128);
        assert_eq!(TokenContract::allowance(env.clone(), alice.clone(), bob.clone()), 10i128);
    }

    #[test]
    fn test_expiry_of_allowance() {
        let env = setup_env();
        let admin = Address::random(&env);
        let alice = Address::random(&env);
        let bob = Address::random(&env);

        TokenContract::initialize(env.clone(), admin.clone(), "T".to_string(), "T".to_string(), 2u32, 0i128);
        env.set_signer(&admin);
        TokenContract::mint(env.clone(), alice.clone(), 50i128);

        env.set_signer(&alice);
        // set allowance that expires at ledger 2
        TokenContract::approve(env.clone(), alice.clone(), bob.clone(), 10i128, 2u32);

        // ledger currently 1
        assert_eq!(TokenContract::allowance(env.clone(), alice.clone(), bob.clone()), 10i128);

        // advance ledger to 3
        env.ledger().set(Ledger { sequence: 3, timestamp: 0, protocol_version: 1 });

        // now allowance treated as 0
        assert_eq!(TokenContract::allowance(env.clone(), alice.clone(), bob.clone()), 0i128);
    }
}
