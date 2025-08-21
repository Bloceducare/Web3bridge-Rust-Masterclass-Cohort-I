use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, String, Symbol};
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils};

use crate::error::ContractError;
use crate::storage::{
    read_allowance, read_balance, receive_balance, spend_allowance, spend_balance, write_allowance,
};
use crate::token::TokenTrait;
use crate::types::DataKey;
use crate::utils::check_nonnegative_amount;

#[contract]
pub struct Token;

#[contractimpl]
impl TokenTrait for Token {
    fn __constructor(
        e: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) -> Result<(), ContractError> {
        if e.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitializedError);
        }

        // Set admin
        e.storage().instance().set(&DataKey::Admin, &admin);

        // Set metadata
        let metadata = TokenMetadata {
            decimal,
            name,
            symbol,
        };
        e.storage()
            .instance()
            .set(&symbol_short!("METADATA"), &metadata);

        Ok(())
    }

    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        read_allowance(&e, from, spender).amount
    }

    fn approve(
        e: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) -> Result<(), ContractError> {
        from.require_auth();
        check_nonnegative_amount(amount)?;

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);

        Ok(())
    }

    fn balance(e: Env, id: Address) -> i128 {
        read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();
        check_nonnegative_amount(amount)?;

        spend_balance(&e, from.clone(), amount)?;
        receive_balance(&e, to.clone(), amount)?;
        TokenUtils::new(&e).events().transfer(from, to, amount);

        Ok(())
    }

    fn transfer_from(
        e: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        spender.require_auth();
        check_nonnegative_amount(amount)?;

        spend_allowance(&e, from.clone(), spender, amount)?;
        spend_balance(&e, from.clone(), amount)?;
        receive_balance(&e, to.clone(), amount)?;
        TokenUtils::new(&e).events().transfer(from, to, amount);

        Ok(())
    }

    fn burn(e: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();
        check_nonnegative_amount(amount)?;

        spend_balance(&e, from.clone(), amount)?;
        TokenUtils::new(&e).events().burn(from, amount);

        Ok(())
    }

    fn burn_from(
        e: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        spender.require_auth();
        check_nonnegative_amount(amount)?;

        spend_allowance(&e, from.clone(), spender, amount)?;
        spend_balance(&e, from.clone(), amount)?;
        TokenUtils::new(&e).events().burn(from, amount);

        Ok(())
    }

    fn decimals(e: Env) -> Result<u32, ContractError> {
        let metadata = e
            .storage()
            .instance()
            .get::<Symbol, TokenMetadata>(&symbol_short!("METADATA"))
            .ok_or(ContractError::NotInitializedError)?;
        Ok(metadata.decimal)
    }

    fn name(e: Env) -> Result<String, ContractError> {
        let metadata = e
            .storage()
            .instance()
            .get::<Symbol, TokenMetadata>(&symbol_short!("METADATA"))
            .ok_or(ContractError::NotInitializedError)?;
        Ok(metadata.name)
    }

    fn symbol(e: Env) -> Result<String, ContractError> {
        let metadata = e
            .storage()
            .instance()
            .get::<Symbol, TokenMetadata>(&symbol_short!("METADATA"))
            .ok_or(ContractError::NotInitializedError)?;
        Ok(metadata.symbol)
    }

    fn mint(e: Env, to: Address, amount: i128) -> Result<(), ContractError> {
        let admin = e
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Admin)
            .ok_or(ContractError::NotInitializedError)?;
        admin.require_auth();

        check_nonnegative_amount(amount)?;
        receive_balance(&e, to.clone(), amount)?;
        TokenUtils::new(&e).events().mint(admin, to, amount);

        Ok(())
    }

    fn set_admin(e: Env, new_admin: Address) -> Result<(), ContractError> {
        let admin = e
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Admin)
            .ok_or(ContractError::NotInitializedError)?;
        admin.require_auth();

        e.storage().instance().set(&DataKey::Admin, &new_admin);
        TokenUtils::new(&e).events().set_admin(admin, new_admin);

        Ok(())
    }

    fn admin(e: Env) -> Result<Address, ContractError> {
        e.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::Admin)
            .ok_or(ContractError::NotInitializedError)
    }
}
