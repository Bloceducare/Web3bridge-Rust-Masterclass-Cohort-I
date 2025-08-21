use crate::errors::{InsufficientAllowance, InsufficientBalance, TransferToSelfOrZeroAddress, ZeroAmount};
use crate::{errors, ierc20};
use alloc::string::String;
use alloc::vec::Vec;
use alloy_primitives::U160;
use errors::ERC20Errors;
use ierc20::IERC20;
use stylus_sdk::storage::{StorageAddress, StorageMap, StorageString, StorageU256};
use stylus_sdk::{
    ArbResult,
    alloy_primitives::{Address, U256},
    prelude::*,
};

#[storage]
pub struct ERC20 {
    name: StorageString,
    symbol: StorageString,
    owner: StorageAddress,
    total_supply: StorageU256,
    balance: StorageMap<Address, StorageU256>,
    allowance: StorageMap<Address, StorageMap<Address, StorageU256>>,
}

#[public]
impl IERC20 for ERC20 {
    fn init(
        &mut self,
        name: String,
        symbol: String,
        initial_supply: u128,
    ) -> Result<bool, Vec<u8>> {
        self.name.set_str(name);
        self.symbol.set_str(symbol);
        self._mint(self.vm().msg_sender(), U256::from(initial_supply));
        self.owner.set(self.vm().msg_sender());
        Ok(true)
    }

    fn name(&self) -> String {
        self.name.get_string()
    }

    fn symbol(&self) -> String {
        self.symbol.get_string()
    }

    fn decimals(&self) -> u8 {
        18
    }

    fn total_supply(&self) -> u128 {
        self.total_supply.get().try_into().unwrap()
    }

    fn balance_of(&self, owner: Address) -> u128 {
        self._balance(owner).try_into().unwrap()
    }

    fn allowance(&self, owner: Address, spender: Address) -> u128 {
        self._get_allowance(owner, spender).try_into().unwrap()
    }

    fn transfer(&mut self, to: Address, value: u128) -> ArbResult {
        self._check_zero_transaction(value.try_into().unwrap()).ok_or(self._throw_zero_amount())?;
        let from: Address = self.vm().msg_sender();
        self._compare_addresses(from, to).ok_or(self._throw_address_error())?;
        let bal: U256 = self._balance(from);
        self._check_balance(bal, value.try_into().unwrap())
            .ok_or(self._throw_insufficient_balance(from, value.try_into().unwrap()))?;
        self._transfer(from, bal, to, value.try_into().unwrap());
        Ok(vec![true.into()])
    }

    fn transfer_from(&mut self, owner: Address, to: Address, value: u128) -> ArbResult {
        self._check_zero_transaction(value.try_into().unwrap()).ok_or(self._throw_zero_amount())?;
        self._compare_addresses(owner, to).ok_or(self._throw_address_error())?;
        let spender: Address = self.vm().msg_sender();
        let allowance: U256 = self._get_allowance(owner, spender);

        if self
            ._check_allowance(allowance, U256::from(value))
            .is_none()
        {
            return Err(self
                ._throw_insufficient_allowance(spender, allowance)
                .try_into()
                .unwrap());
        }

        let owner_balance = self.balance.get(owner);

        if self
            ._check_balance(owner_balance, value.try_into().unwrap())
            .is_none()
        {
            return Err(self
                ._throw_insufficient_balance(owner, allowance)
                .try_into()
                .unwrap());
        }
        self._update_allowance(owner, spender, allowance, value.try_into().unwrap());
        self._transfer(owner, owner_balance, to, U256::from(value));
        Ok(vec![true.into()])
    }

    fn approve(&mut self, spender: Address, value: u128) -> ArbResult {
        self._check_zero_transaction(value.try_into().unwrap()).ok_or(self._throw_zero_amount())?;
        let from: Address = self.vm().msg_sender();
        // confirm approval is not to self
        self._compare_addresses(from, spender).ok_or(self._throw_address_error())?;
        // check balance
        self._check_balance(self._balance(from), value.try_into().unwrap())
            .ok_or(self._throw_insufficient_balance(from, value.try_into().unwrap()))?;
        self.allowance
            .setter(from)
            .setter(spender)
            .set(value.try_into().unwrap());
        Ok(vec![true.into()])
    }
}

impl ERC20 {
    fn _throw_insufficient_balance(&self, address: Address, value: U256) -> ERC20Errors {
        ERC20Errors::InsufficientBalance(InsufficientBalance {
            account: address,
            amount: value,
        })
    }

    fn _throw_zero_amount(&self,) -> ERC20Errors {
        ERC20Errors::ZeroAmount(ZeroAmount{})
    }

    fn _throw_address_error(&self,) -> ERC20Errors {
        ERC20Errors::TransferToZeroAddress(TransferToSelfOrZeroAddress {})
    }

    fn _throw_insufficient_allowance(&self, spender: Address, value: U256) -> ERC20Errors {
        ERC20Errors::InsufficientAllowance(InsufficientAllowance {
            spender,
            amount: value,
        })
    }
}

impl ERC20 {
    fn _transfer(&mut self, from: Address, bal: U256, to: Address, value: U256) -> bool {
        self.balance.insert(from, bal - value);
        self.balance.insert(to, self.balance.get(to) + value);
        true
    }

    fn _check_zero_transaction(&self, value: U256) -> Option<bool> {
        if value != U256::from(0) {
            return Some(true);
        }
        None
    }
    fn _get_allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowance.get(owner).get(spender)
    }
    fn _compare_addresses(&self, user: Address, other_address: Address) -> Option<bool> {
        if user != other_address && user != Address::from(U160::from(0)) {
            return Some(true);
        }
        None
    }
    fn _update_allowance(
        &mut self,
        owner: Address,
        spender: Address,
        allowance: U256,
        value: U256,
    ) {
        self.allowance
            .setter(owner)
            .setter(spender)
            .set(allowance - value);
    }

    fn _check_allowance(&self, allowance: U256, amount: U256) -> Option<bool> {
        if allowance >= amount {
            return Some(true);
        }
        None
    }

    fn _check_balance(&self, balance: U256, value: U256) -> Option<bool> {
        //check balance
        if balance >= value {
            return Some(true);
        }
        None
    }

    fn _mint(&mut self, to: Address, value: U256) {
        let bal = self.balance.get(to);
        self.balance.insert(to, bal + value);
        self.total_supply.set(self.total_supply.get() + value);
    }
    fn _balance(&self, address: Address) -> U256 {
        self.balance.get(address)
    }
}