use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};

/// Event topics as symbols
pub const TRANSFER: Symbol = symbol_short!("transfer");
pub const APPROVE: Symbol = symbol_short!("approve");
pub const BURN: Symbol = symbol_short!("burn");
pub const MINT: Symbol = symbol_short!("mint");

/// Transfer event data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

/// Approval event data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovalEvent {
    pub owner: Address,
    pub spender: Address,
    pub amount: i128,
    pub expiration_ledger: u32,
}

/// Burn event data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnEvent {
    pub from: Address,
    pub amount: i128,
}

/// Mint event data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintEvent {
    pub to: Address,
    pub amount: i128,
}

/// Event emission functions
pub mod events {
    use super::*;

    /// Emit a transfer event
    pub fn emit_transfer(env: &Env, from: Address, to: Address, amount: i128) {
        let event = TransferEvent { from, to, amount };
        env.events()
            .publish((TRANSFER,), event);
    }

    /// Emit an approval event
    pub fn emit_approval(
        env: &Env,
        owner: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        let event = ApprovalEvent {
            owner,
            spender,
            amount,
            expiration_ledger,
        };
        env.events()
            .publish((APPROVE,), event);
    }

    /// Emit a burn event
    pub fn emit_burn(env: &Env, from: Address, amount: i128) {
        let event = BurnEvent { from, amount };
        env.events()
            .publish((BURN,), event);
    }

    /// Emit a mint event
    pub fn emit_mint(env: &Env, to: Address, amount: i128) {
        let event = MintEvent { to, amount };
        env.events()
            .publish((MINT,), event);
    }
}