use soroban_sdk::{contracttype, Address, Env, symbol_short};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproveEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnEvent {
    pub from: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintEvent {
    pub admin: Address,
    pub to: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClawbackEvent {
    pub admin: Address,
    pub from: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Approve(ApproveEvent),
    Transfer(TransferEvent),
    Burn(BurnEvent),
    Mint(MintEvent),
    Clawback(ClawbackEvent),
}

impl Event {
    pub fn publish(&self, env: &Env) {
        match self {
            Event::Approve(event) => {
                env.events().publish((symbol_short!("approve"), &event.from, &event.to), event.amount);
            }
            Event::Transfer(event) => {
                env.events().publish((symbol_short!("transfer"), &event.from, &event.to), event.amount);
            }
            Event::Burn(event) => {
                env.events().publish((symbol_short!("burn"), &event.from), event.amount);
            }
            Event::Mint(event) => {
                env.events().publish((symbol_short!("mint"), &event.admin, &event.to), event.amount);
            }
            Event::Clawback(event) => {
                env.events().publish((symbol_short!("clawback"), &event.admin, &event.from), event.amount);
            }
        }
    }
}