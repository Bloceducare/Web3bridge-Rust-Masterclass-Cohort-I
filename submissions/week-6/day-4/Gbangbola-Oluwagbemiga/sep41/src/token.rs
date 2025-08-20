use crate::error::ContractError;
use soroban_sdk::{Address, Env, String};

pub trait TokenTrait {
    fn __constructor(
        e: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) -> Result<(), ContractError>;

    fn allowance(e: Env, from: Address, spender: Address) -> i128;

    fn approve(
        e: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) -> Result<(), ContractError>;

    fn balance(e: Env, id: Address) -> i128;

    fn transfer(e: Env, from: Address, to: Address, amount: i128) -> Result<(), ContractError>;

    fn transfer_from(
        e: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError>;

    fn burn(e: Env, from: Address, amount: i128) -> Result<(), ContractError>;

    fn burn_from(
        e: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), ContractError>;

    fn decimals(e: Env) -> Result<u32, ContractError>;

    fn name(e: Env) -> Result<String, ContractError>;

    fn symbol(e: Env) -> Result<String, ContractError>;

    fn mint(e: Env, to: Address, amount: i128) -> Result<(), ContractError>;

    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError>;

    fn admin(e: Env) -> Result<Address, ContractError>;
}
