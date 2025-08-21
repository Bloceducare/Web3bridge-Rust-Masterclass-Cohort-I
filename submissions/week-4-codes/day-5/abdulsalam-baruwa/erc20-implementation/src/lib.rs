//! ERC20 token implementation for Arbitrum Stylus.
//!
//! This implementation provides a compliant ERC20 token with standard functionality
//! and security features optimized for the Stylus runtime environment.

#![cfg_attr(all(not(feature = "export-abi"), not(test)), no_main)]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolError};
use stylus_sdk::{
    prelude::*,
    storage::{StorageMap, StorageString, StorageU256},
};

// Note: Stylus SDK provides its own global allocator

// Define the ERC20 interface using Solidity ABI
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    error InsufficientBalance(address from, uint256 have, uint256 want);
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
    error ZeroAddress();
}

/// ERC20 token storage layout.
#[entrypoint]
#[storage]
pub struct Erc20 {
    name: StorageString,
    symbol: StorageString,
    decimals: StorageU256,
    total_supply: StorageU256,
    balances: StorageMap<Address, StorageU256>,
    allowances: StorageMap<Address, StorageMap<Address, StorageU256>>,
}

/// ERC20 token implementation.
#[public]
impl Erc20 {
    /// Initialize the token with name, symbol, decimals, and initial supply.
    pub fn initialize(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
    ) -> Result<(), Vec<u8>> {
        if !self.total_supply.get().is_zero() {
            return Err(b"Already initialized".to_vec());
        }

        self.name.set_str(&name);
        self.symbol.set_str(&symbol);
        self.decimals.set(U256::from(decimals));
        self.total_supply.set(initial_supply);

        let deployer = self.vm().msg_sender();
        self.balances.setter(deployer).set(initial_supply);
        log(
            self.vm(),
            Transfer {
                from: Address::ZERO,
                to: deployer,
                value: initial_supply,
            },
        );

        Ok(())
    }

    /// Returns the name of the token
    pub fn name(&self) -> Result<String, Vec<u8>> {
        Ok(self.name.get_string())
    }

    /// Returns the symbol of the token
    pub fn symbol(&self) -> Result<String, Vec<u8>> {
        Ok(self.symbol.get_string())
    }

    /// Returns the number of decimal places
    pub fn decimals(&self) -> Result<u8, Vec<u8>> {
        Ok(self.decimals.get().to::<u8>())
    }

    /// Returns the total supply of tokens
    pub fn total_supply(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_supply.get())
    }

    /// Returns the balance of the specified address
    ///
    /// # Arguments
    /// * `account` - The address to check the balance of
    pub fn balance_of(&self, account: Address) -> Result<U256, Vec<u8>> {
        Ok(self.balances.get(account))
    }

    /// Transfer tokens from caller to recipient
    ///
    /// # Arguments
    /// * `to` - The recipient address
    /// * `amount` - The amount to transfer
    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let from = self.vm().msg_sender();
        self._transfer(from, to, amount)?;
        Ok(true)
    }

    /// Returns the allowance of spender for owner's tokens
    ///
    /// # Arguments
    /// * `owner` - The token owner
    /// * `spender` - The address allowed to spend tokens
    pub fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Vec<u8>> {
        Ok(self.allowances.getter(owner).get(spender))
    }

    /// Approve spender to spend tokens on behalf of caller
    ///
    /// # Arguments
    /// * `spender` - The address to approve
    /// * `amount` - The amount to approve
    pub fn approve(&mut self, spender: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let owner = self.vm().msg_sender();
        self._approve(owner, spender, amount)?;
        Ok(true)
    }

    /// Transfer tokens from one address to another using allowance
    ///
    /// # Arguments
    /// * `from` - The sender address
    /// * `to` - The recipient address  
    /// * `amount` - The amount to transfer
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<bool, Vec<u8>> {
        let spender = self.vm().msg_sender();

        let current_allowance = self.allowances.getter(from).get(spender);
        if current_allowance < amount {
            return Err(InsufficientAllowance {
                owner: from,
                spender,
                have: current_allowance,
                want: amount,
            }
            .abi_encode());
        }

        let new_allowance = current_allowance - amount;
        self.allowances
            .setter(from)
            .setter(spender)
            .set(new_allowance);

        self._transfer(from, to, amount)?;

        Ok(true)
    }

    /// Increase the allowance of spender
    ///
    /// # Arguments
    /// * `spender` - The address to increase allowance for
    /// * `added_value` - The amount to increase allowance by
    pub fn increase_allowance(
        &mut self,
        spender: Address,
        added_value: U256,
    ) -> Result<bool, Vec<u8>> {
        let owner = self.vm().msg_sender();
        let current_allowance = self.allowances.getter(owner).get(spender);
        let new_allowance = current_allowance + added_value;
        self._approve(owner, spender, new_allowance)?;
        Ok(true)
    }

    /// Decrease the allowance of spender
    ///
    /// # Arguments
    /// * `spender` - The address to decrease allowance for
    /// * `subtracted_value` - The amount to decrease allowance by
    pub fn decrease_allowance(
        &mut self,
        spender: Address,
        subtracted_value: U256,
    ) -> Result<bool, Vec<u8>> {
        let owner = self.vm().msg_sender();
        let current_allowance = self.allowances.getter(owner).get(spender);

        if current_allowance < subtracted_value {
            return Err(InsufficientAllowance {
                owner,
                spender,
                have: current_allowance,
                want: subtracted_value,
            }
            .abi_encode());
        }

        let new_allowance = current_allowance - subtracted_value;
        self._approve(owner, spender, new_allowance)?;
        Ok(true)
    }
}

/// Internal helper functions
impl Erc20 {
    /// Internal transfer function with safety checks
    ///
    /// # Arguments
    /// * `from` - The sender address
    /// * `to` - The recipient address
    /// * `amount` - The amount to transfer
    fn _transfer(&mut self, from: Address, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        // Check for zero address
        if to == Address::ZERO {
            return Err(ZeroAddress {}.abi_encode());
        }

        let from_balance = self.balances.get(from);
        if from_balance < amount {
            return Err(InsufficientBalance {
                from,
                have: from_balance,
                want: amount,
            }
            .abi_encode());
        }

        self.balances.setter(from).set(from_balance - amount);
        let to_balance = self.balances.get(to);
        self.balances.setter(to).set(to_balance + amount);

        // Emit Transfer event
        log(
            self.vm(),
            Transfer {
                from,
                to,
                value: amount,
            },
        );

        Ok(())
    }

    /// Internal approve function
    ///
    /// # Arguments
    /// * `owner` - The token owner
    /// * `spender` - The address to approve
    /// * `amount` - The amount to approve
    fn _approve(&mut self, owner: Address, spender: Address, amount: U256) -> Result<(), Vec<u8>> {
        // Check for zero addresses
        if owner == Address::ZERO || spender == Address::ZERO {
            return Err(ZeroAddress {}.abi_encode());
        }

        self.allowances.setter(owner).setter(spender).set(amount);

        // Emit Approval event
        log(
            self.vm(),
            Approval {
                owner,
                spender,
                value: amount,
            },
        );

        Ok(())
    }
}

// Include tests module only when not targeting WASM
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
