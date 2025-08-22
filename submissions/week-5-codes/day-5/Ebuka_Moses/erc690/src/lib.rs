#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

use alloy_primitives::Address;
use alloy_sol_types::{sol, SolError};
use openzeppelin_stylus::{
    access::control::{self, AccessControl, IAccessControl},
    utils::introspection::erc165::IErc165,
};

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::U256,
    msg,
    prelude::*,
    storage::{StorageBool, StorageMap, StorageU256},
};

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol! {
    event Transfer();
}
sol! {
    error ERC6909InsufficientBalance(address sender,uint256 balance,uint256 needed ,uint256 id);

    error ERC6909InsufficientAllowance(address spender,uint256 allowance,uint256 needed,uint256 id);

    error ERC6909InvalidApprover(address approver);

    error ERC6909InvalidReciever(address reciever);

    error ERC6909InvalidSender(address sender);

    error ERC6909InvalidSpender(address spender);

}

#[derive(SolidityError)]
pub enum ERC6909Error {
    ERC6909InsufficientAllowance(ERC6909InsufficientAllowance),
    ERC6909InsufficientBalance(ERC6909InsufficientBalance),
    ERC6909InvalidApprover(ERC6909InvalidApprover),
    ERC6909InvalidReciever(ERC6909InvalidReciever),
    ERC6909InvalidSender(ERC6909InvalidSender),
    ERC6909InvalidSpender(ERC6909InvalidSpender),
}

impl From<control::Error> for ERC6909Error {
    fn from(value: control::Error) -> Self {
        match value {
            control::Error::UnauthorizedAccount(e) => {
                ERC6909Error::ERC6909InvalidApprover(ERC6909InvalidApprover {
                    approver: e.account,
                })
            }
            control::Error::BadConfirmation(e) => {
                ERC6909Error::ERC6909InvalidSender(ERC6909InvalidSender {
                    sender: Address::ZERO,
                })
            }
        }
    }
}

#[storage]
#[entrypoint]
pub struct ERC6909 {
    _balances: StorageMap<Address, StorageMap<U256, StorageU256>>,
    _operatorApprovals: StorageMap<Address, StorageMap<Address, StorageBool>>,
    _allowances: StorageMap<Address, StorageMap<Address, StorageMap<U256, StorageU256>>>,
}

#[public]
impl ERC6909 {
    pub fn balanceOf(&mut self, address: Address, id: U256) -> U256 {
        self._balances.get(address).get(id)
    }

    pub fn allowance(self, owner: Address, spender: Address, id: U256) -> U256 {
        self._allowances.get(owner).get(spender).get(id)
    }

    pub fn isOperator(&mut self, owner: Address, spender: Address) {
        let _ = self._operatorApprovals.get(owner).get(spender);
    }

    pub fn approve(&mut self, spender: Address, id: U256, amount: U256) -> bool {
        let owner = msg::sender();
        self._approve(owner, spender, id, amount);
        true
    }

    pub fn setOperator(self, spender: Address, approved: bool) -> bool {
        todo!("not implemented");
        return true;
    }

    pub fn transfer(&mut self, reciever: Address, id: U256, amount: U256) -> bool {
        let sender = msg::sender();
        self._transfer(sender, reciever, id, amount);
        true
    }

    pub fn transferFrom(self, sender: Address, id: U256, amount: U256) -> bool {
        todo!("not implemented");
        return true;
    }

    pub fn _mint(&mut self, to: Address, id: U256, amount: U256) -> Result<(), ERC6909Error> {
        if to.is_zero() {
            return Err(ERC6909Error::ERC6909InvalidReciever(
                ERC6909InvalidReciever { reciever: to },
            ));
        }
        self._update(Address::ZERO, to, id, amount)?;
        Ok(())
    }

    pub fn _update(
        &mut self,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), ERC6909Error> {
        if !from.is_zero() {
            let fromBalance = self._balances.get(from).get(id);
            if fromBalance < amount {
                return Err(ERC6909Error::ERC6909InsufficientBalance(
                    ERC6909InsufficientBalance {
                        sender: from,
                        balance: fromBalance,
                        needed: amount,
                        id,
                    },
                ));
            }
            self._balances
                .setter(from)
                .setter(id)
                .set(fromBalance - amount);
        }
        if !to.is_zero() {
            let toBalance = self._balances.get(to).get(id);
            self._balances.setter(to).setter(id).set(toBalance + amount);
        }
        Ok(())
    }

    pub fn _transfer(
        &mut self,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), ERC6909Error> {
        if from.is_zero() {
            return Err(ERC6909Error::ERC6909InvalidSender(ERC6909InvalidSender {
                sender: from,
            }));
        }
        if to.is_zero() {
            return Err(ERC6909Error::ERC6909InvalidReciever(
                ERC6909InvalidReciever { reciever: to },
            ));
        }
        self._update(from, to, id, amount)?;
        Ok(())
    }

    pub fn _approve(
        &mut self,
        owner: Address,
        spender: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), ERC6909Error> {
        if owner.is_zero() {
            return Err(ERC6909Error::ERC6909InvalidApprover(
                ERC6909InvalidApprover { approver: owner },
            ));
        }
        if spender.is_zero() {
            return Err(ERC6909Error::ERC6909InvalidSpender(ERC6909InvalidSpender {
                spender,
            }));
        }

        self._allowances
            .setter(owner)
            .setter(spender)
            .setter(id)
            .set(amount);
        Ok(())
    }

    pub fn _setOperator(
        &mut self,
        owner: Address,
        spender: Address,
        approved: bool,
    ) -> Result<(), ERC6909Error> {
        if owner.is_zero() {
            return Err(ERC6909Error::ERC6909InvalidSpender(ERC6909InvalidSpender {
                spender: owner,
            }));
        }
        if spender.is_zero() {
            return Err(ERC6909Error::ERC6909InvalidSpender(ERC6909InvalidSpender {
                spender,
            }));
        }
        self._operatorApprovals
            .setter(owner)
            .setter(spender)
            .set(approved);
        Ok(())
    }

    pub fn _spendAllowance(
        &mut self,
        owner: Address,
        spender: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), ERC6909Error> {
        let allowance = self._allowances.get(owner).get(spender).get(id);
        if allowance < amount {
            return Err(ERC6909Error::ERC6909InsufficientAllowance(
                ERC6909InsufficientAllowance {
                    needed: spender,
                    id: allowance,
                },
            ));
        }
        self._allowances
            .setter(owner)
            .setter(spender)
            .setter(id)
            .set(allowance - amount);
        Ok(())
    }
}
