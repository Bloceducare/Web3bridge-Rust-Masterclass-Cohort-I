

#![allow(unused_imports)]
extern crate alloc;

use alloc::string::String;
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use core::marker::PhantomData;
use stylus_sdk::{evm, prelude::*};

/// Generic params trait (parity with ERC-20 generic pattern)
pub trait Erc6909Params {
    const NAME: &'static str;
    const SYMBOL: &'static str;
}

sol_storage! {
    /// Erc6909 implements multi-token balances, per-token allowances,
    /// operator approvals and per-id total supplies.
    pub struct Erc6909<T> {
        /// balances[id][account] => amount
        mapping(uint256 => mapping(address => uint256)) balances;
        /// allowances[owner][spender][id] => amount
        mapping(address => mapping(address => mapping(uint256 => uint256))) allowances;
        /// operator approvals: approvals[owner][operator] => bool
        mapping(address => mapping(address => bool)) operator_approvals;
        /// total supply per token ID
        mapping(uint256 => uint256) total_supplies;
        /// keep generic phantom
        PhantomData<T> phantom;
    }
}

// Events & errors (Alloy `sol!` declarations)
sol! {
    #[derive(Debug)]
    event TransferSingle(
        address indexed operator,
        address indexed from,
        address indexed to,
        uint256 id,
        uint256 amount
    );

    #[derive(Debug)]
    event ApprovalSingle(
        address indexed owner,
        address indexed spender,
        uint256 indexed id,
        uint256 amount
    );

    #[derive(Debug)]
    event OperatorApproval(address indexed owner, address indexed operator, bool approved);

    #[derive(Debug)]
    error InsufficientBalance(address from, uint256 id, uint256 have, uint256 want);

    #[derive(Debug)]
    error InsufficientAllowance(address owner, address spender, uint256 id, uint256 have, uint256 want);

    #[derive(Debug)]
    error ZeroAddress();

    #[derive(Debug)]
    error ZeroAmount();
}

// Rust enum mapping to the Alloy/sol errors
#[derive(SolidityError, Debug)]
pub enum Erc6909Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
    ZeroAddress(ZeroAddress),
    ZeroAmount(ZeroAmount),
}

/* ========= Internal helpers (no `#[public]`) ========= */
impl<T: Erc6909Params> Erc6909<T> {
    /// Internal transfer between two accounts for token `id`.
    pub fn _transfer(
        &mut self,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), Erc6909Error> {
        if from == Address::ZERO || to == Address::ZERO {
            return Err(Erc6909Error::ZeroAddress(ZeroAddress {}));
        }

        // Use intermediate binding to hold the map across the borrow
        let mut balances_for_id = self.balances.setter(id);
        let mut from_bal = balances_for_id.setter(from);
        let old_from = from_bal.get();
        if old_from < amount {
            return Err(Erc6909Error::InsufficientBalance(InsufficientBalance {
                from,
                id,
                have: old_from,
                want: amount,
            }));
        }
        from_bal.set(old_from - amount);

        // credit to
        // reuse balances_for_id (can't reuse the previous `from_bal` binding once dropped)
        // create a fresh setter for `to`
        let mut balances_for_id = self.balances.setter(id);
        let mut to_bal = balances_for_id.setter(to);
        let cur_to = to_bal.get();
        to_bal.set(cur_to + amount);

        // emit event
        log(
            self.vm(),
            TransferSingle {
                operator: self.vm().msg_sender(),
                from,
                to,
                id,
                amount,
            },
        );

        Ok(())
    }

    /// Internal allowance spending for (owner, spender, id)
    pub fn _spend_allowance(
        &mut self,
        owner: Address,
        spender: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), Erc6909Error> {
        // split into steps to avoid temporary drop issues
        let mut owner_allowances = self.allowances.setter(owner);
        let mut spender_allowances = owner_allowances.setter(spender);
        let mut allowance_slot = spender_allowances.setter(id);

        let current = allowance_slot.get();
        if current < amount {
            return Err(Erc6909Error::InsufficientAllowance(InsufficientAllowance {
                owner,
                spender,
                id,
                have: current,
                want: amount,
            }));
        }
        allowance_slot.set(current - amount);

        // Emit updated allowance
        log(
            self.vm(),
            ApprovalSingle {
                owner,
                spender,
                id,
                amount: current - amount,
            },
        );
        Ok(())
    }

    /// Internal mint (updates balance and total supply)
    pub fn _mint(&mut self, to: Address, id: U256, amount: U256) -> Result<(), Erc6909Error> {
        if to == Address::ZERO {
            return Err(Erc6909Error::ZeroAddress(ZeroAddress {}));
        }

        // update balance
        let mut balances_for_id = self.balances.setter(id);
        let mut bal = balances_for_id.setter(to);
        let cur = bal.get();
        bal.set(cur + amount);

        // update supply
        let mut supply_slot = self.total_supplies.setter(id);
        let cur_supply = supply_slot.get();
        supply_slot.set(cur_supply + amount);

        // event
        log(
            self.vm(),
            TransferSingle {
                operator: self.vm().msg_sender(),
                from: Address::ZERO,
                to,
                id,
                amount,
            },
        );

        Ok(())
    }

    /// Internal burn (updates balance and total supply)
    pub fn _burn(&mut self, from: Address, id: U256, amount: U256) -> Result<(), Erc6909Error> {
        // update balance
        let mut balances_for_id = self.balances.setter(id);
        let mut bal = balances_for_id.setter(from);
        let old = bal.get();
        if old < amount {
            return Err(Erc6909Error::InsufficientBalance(InsufficientBalance {
                from,
                id,
                have: old,
                want: amount,
            }));
        }
        bal.set(old - amount);

        // update supply
        let mut supply_slot = self.total_supplies.setter(id);
        let cur_supply = supply_slot.get();
        supply_slot.set(cur_supply - amount);

        // event
        log(
            self.vm(),
            TransferSingle {
                operator: self.vm().msg_sender(),
                from,
                to: Address::ZERO,
                id,
                amount,
            },
        );

        Ok(())
    }
}

/* ========= Public ERC-6909 API ========= */
#[public]
impl<T: Erc6909Params> Erc6909<T> {
    /// Optional metadata accessors
    pub fn name(&self) -> String {
        T::NAME.into()
    }
    pub fn symbol(&self) -> String {
        T::SYMBOL.into()
    }

    /// Per-token total supply
    pub fn total_supply(&self, id: U256) -> U256 {
        self.total_supplies.get(id)
    }

    /// Balance of `owner` for a given token id
    pub fn balance_of(&self, owner: Address, id: U256) -> U256 {
        self.balances.getter(id).get(owner)
    }

    /// Per-token allowance getter
    pub fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256 {
        self.allowances.getter(owner).getter(spender).get(id)
    }

    /// Approve a spender for a specific token id
    pub fn approve(&mut self, spender: Address, id: U256, amount: U256) -> bool {
        let owner = self.vm().msg_sender();

        let mut owner_allowances = self.allowances.setter(owner);
        let mut spender_allowances = owner_allowances.setter(spender);
        spender_allowances.insert(id, amount);

        log(
            self.vm(),
            ApprovalSingle {
                owner,
                spender,
                id,
                amount,
            },
        );
        true
    }

    /// Set/unset an operator to manage **all** token ids of the caller
    pub fn set_operator(&mut self, operator: Address, approved: bool) -> bool {
        let owner = self.vm().msg_sender();

        let mut owner_ops = self.operator_approvals.setter(owner);
        owner_ops.insert(operator, approved);

        log(
            self.vm(),
            OperatorApproval {
                owner,
                operator,
                approved,
            },
        );
        true
    }

    /// Query operator approval
    pub fn operator_approval(&self, owner: Address, operator: Address) -> bool {
        self.operator_approvals.getter(owner).get(operator)
    }

    /// Transfer respecting per-id allowances and operator approvals
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Erc6909Error> {
        let caller = self.vm().msg_sender();

        // If caller isn't `from` and not a global operator, consume allowance
        if caller != from && !self.operator_approvals.getter(from).get(caller) {
            self._spend_allowance(from, caller, id, amount)?;
        }

        self._transfer(from, to, id, amount)?;
        Ok(true)
    }

    /// Mint that updates total supply — convenience wrapper to call internal mint.
    pub fn mint(&mut self, to: Address, id: U256, amount: U256) -> Result<bool, Erc6909Error> {
        self._mint(to, id, amount)?;
        Ok(true)
    }

    /// Burn that updates total supply — convenience wrapper to call internal burn.
    pub fn burn(&mut self, from: Address, id: U256, amount: U256) -> Result<bool, Erc6909Error> {
        self._burn(from, id, amount)?;
        Ok(true)
    }
}
