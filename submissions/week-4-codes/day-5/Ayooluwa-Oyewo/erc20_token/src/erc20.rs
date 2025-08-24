#![allow(unused_imports)]
extern crate alloc;

use alloc::string::String;
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use core::marker::PhantomData;
use stylus_sdk::{evm, prelude::*};

/// Parameters to configure name/symbol/decimals at compile time
pub trait Erc20Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
    const DECIMALS: u8;
}

// Solidity-compatible storage layout for ERC-20
sol_storage! {
    /// Erc20 implements all ERC-20 methods and state.
    pub struct Erc20<T> {
        /// Maps users to balances
        mapping(address => uint256) balances;
        /// Maps users to a mapping of each spender's allowance
        mapping(address => mapping(address => uint256)) allowances;
        /// The total supply of the token
        uint256 total_supply;
        /// Used to allow [`Erc20Params`]
        PhantomData<T> phantom;
    }
}

// Declare standard ERC-20 events & errors using Alloy's `sol!`
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
    #[derive(Debug)]
    event Approval(address indexed owner, address indexed spender, uint256 value);

    #[derive(Debug)]
    error InsufficientBalance(address from, uint256 have, uint256 want);
    #[derive(Debug)]
    error InsufficientAllowance(address owner, address spender, uint256 have, uint256 want);
}

/// Rust enum mapping to the Solidity-style errors
#[derive(SolidityError, Debug)]
pub enum Erc20Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
}

impl<T: Erc20Params> Erc20<T> {
    /// Internal transfer between two accounts
    pub fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), Erc20Error> {
        // decrease sender
        let mut sender_balance = self.balances.setter(from);
        let old_sender_balance = sender_balance.get();
        if old_sender_balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from,
                have: old_sender_balance,
                want: value,
            }));
        }
        sender_balance.set(old_sender_balance - value);

        // increase recipient
        let mut to_balance = self.balances.setter(to);
        let new_to_balance = to_balance.get() + value;
        to_balance.set(new_to_balance);

        // emit Transfer
        log(self.vm(), Transfer { from, to, value });
        Ok(())
    }

    /// Mint `value` tokens to `address`
    pub fn mint(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + value;
        balance.set(new_balance);

        self.total_supply.set(self.total_supply.get() + value);

        log(
            self.vm(),
            Transfer {
                from: Address::ZERO,
                to: address,
                value,
            },
        );
        Ok(())
    }

    /// Burn `value` tokens from `address`
    pub fn burn(&mut self, address: Address, value: U256) -> Result<(), Erc20Error> {
        let mut balance = self.balances.setter(address);
        let old_balance = balance.get();
        if old_balance < value {
            return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
                from: address,
                have: old_balance,
                want: value,
            }));
        }
        balance.set(old_balance - value);
        self.total_supply.set(self.total_supply.get() - value);
        log(
            self.vm(),
            Transfer {
                from: address,
                to: Address::ZERO,
                value,
            },
        );
        Ok(())
    }
}

#[public]
impl<T: Erc20Params> Erc20<T> {
    pub fn name(&self) -> String {
        T::NAME.into()
    }
    pub fn symbol(&self) -> String {
        T::SYMBOL.into()
    }
    pub fn decimals(&self) -> u8 {
        T::DECIMALS
    }

    pub fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self._transfer(self.vm().msg_sender(), to, value)?;
        Ok(true)
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Erc20Error> {
        // get msg_sender before any mutable borrow
        let msg_sender = self.vm().msg_sender();
        // check allowance for msg_sender
        let mut sender_allowances = self.allowances.setter(from);
        let mut allowance = sender_allowances.setter(msg_sender);
        let old_allowance = allowance.get();
        if old_allowance < value {
            return Err(Erc20Error::InsufficientAllowance(InsufficientAllowance {
                owner: from,
                spender: msg_sender,
                have: old_allowance,
                want: value,
            }));
        }
        allowance.set(old_allowance - value);
        self._transfer(from, to, value)?;
        Ok(true)
    }

    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        self.allowances
            .setter(self.vm().msg_sender())
            .insert(spender, value);
        log(
            self.vm(),
            Approval {
                owner: self.vm().msg_sender(),
                spender,
                value,
            },
        );
        true
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.getter(owner).get(spender)
    }
}
