use soroban_sdk::{contract, contractimpl, token::Interface as TokenInterface, Address, Env};
use stellar_access::ownable::{self as ownable, Ownable};
use stellar_macros::{default_impl, only_owner, when_not_paused};
use stellar_pausable::pausable::{self as pausable, Pausable};
use stellar_tokens::fungible::{Base, FungibleToken};

#[contract]
pub struct FungibleTokenContract;

#[contractimpl]
impl FungibleTokenContract {
    pub fn __constructor(e: &Env, owner: Address) {
        // Set token metadata
        Base::set_metadata(
            e,
            18, // 18 decimals
            "My Token".into(),
            "TKN".into(),
        );

        // Set the contract owner
        ownable::set_owner(e, &owner);
    }

    #[only_owner]
    pub fn mint(e: &Env, to: Address, amount: i128) {
        Base::mint(e, &to, amount);
    }

    #[only_owner]
    pub fn pause(e: &Env) {
        pausable::pause(e);
    }

    #[only_owner]
    pub fn unpause(e: &Env) {
        pausable::unpause(e);
    }
}

#[default_impl]
#[contractimpl]
impl FungibleToken for FungibleTokenContract {
    type ContractType = Base;
}

#[default_impl]
#[contractimpl]
impl TokenInterface for FungibleTokenContract {
    #[when_not_paused]
    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        Base::transfer(&e, &from, &to, amount);
    }

    #[when_not_paused]
    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        Base::transfer_from(&e, &spender, &from, &to, amount);
    }

    #[when_not_paused]
    fn burn(e: Env, from: Address, amount: i128) {
        Base::burn(&e, &from, amount);
    }

    #[when_not_paused]
    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        Base::burn_from(&e, &spender, &from, amount);
    }

    #[when_not_paused]
    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        Base::approve(&e, &from, &spender, amount, expiration_ledger);
    }

    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        Base::allowance(&e, &from, &spender)
    }

    fn balance(e: Env, id: Address) -> i128 {
        Base::balance(&e, &id)
    }

    fn decimals(e: Env) -> u32 {
        Base::decimals(&e)
    }

    fn name(e: Env) -> soroban_sdk::String {
        Base::name(&e)
    }

    fn symbol(e: Env) -> soroban_sdk::String {
        Base::symbol(&e)
    }
}

#[default_impl]
#[contractimpl]
impl Ownable for FungibleTokenContract {}

#[default_impl]
#[contractimpl]
impl Pausable for FungibleTokenContract {}