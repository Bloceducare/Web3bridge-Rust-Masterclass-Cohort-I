#![no_main]
#![no_std]
extern crate alloc;

use alloy_primitives::{Address, U256};
use stylus_sdk::{
    alloy_sol_types::{sol, SolValue},
    evm, msg,
    prelude::*,
    storage::{StorageBool, StorageMap, StorageUint},
};

// Define events using stylus-sdk macros instead of sol!
#[derive(Debug)]
pub struct TransferSingle {
    pub operator: Address,
    pub from: Address,
    pub to: Address,
    pub id: U256,
    pub amount: U256,
}

#[derive(Debug)]
pub struct ApprovalSingle {
    pub owner: Address,
    pub spender: Address,
    pub id: U256,
    pub amount: U256,
}

#[derive(Debug)]
pub struct OperatorSet {
    pub owner: Address,
    pub operator: Address,
    pub approved: bool,
}

// Define custom errors

sol! {
    error InsufficientBalance();
    error InsufficientAllowance();
    error InvalidOperator();
    error TransferToZeroAddress();
    error TransferFromZeroAddress();
}

#[derive(SolidityError)]
pub enum ERC6909Error {
    InsufficientBalance(InsufficientBalance),
    InsufficientAllowance(InsufficientAllowance),
    InvalidOperator(InvalidOperator),
    TransferToZeroAddress(TransferToZeroAddress),
    TransferFromZeroAddress(TransferFromZeroAddress),
}

// Storage layout for ERC-6909
#[storage]
#[entrypoint]
pub struct ERC6909 {
    /// Balance of tokens: owner => token_id => balance
    balances: StorageMap<Address, StorageMap<U256, StorageUint<256, 4>>>,
    
    /// Allowances: owner => spender => token_id => amount
    allowances: StorageMap<Address, StorageMap<Address, StorageMap<U256, StorageUint<256, 4>>>>,
    
    /// Operator approvals: owner => operator => approved
    operators: StorageMap<Address, StorageMap<Address, StorageBool>>,
    
    /// Total supply per token ID
    total_supplies: StorageMap<U256, StorageUint<256, 4>>,
}


#[public]
impl ERC6909 {
    /// Returns the total supply of a specific token ID
    pub fn total_supply(&self, id: U256) -> U256 {
        self.total_supplies.get(id).clone()
    }

    /// Returns the balance of an owner for a specific token ID
    pub fn balance_of(&self, owner: Address, id: U256) -> U256 {
        self.balances.getter(owner).getter(id).get()
    }

    /// Returns the allowance of a spender for a specific token ID from an owner
    pub fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256 {
        self.allowances.getter(owner).getter(spender).getter(id).get()
    }

    /// Returns whether an operator is approved for all tokens of an owner
    pub fn is_operator(&self, owner: Address, operator: Address) -> bool {
        self.operators.getter(owner).getter(operator).get()
    }

    /// Transfers tokens from one address to another
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, ERC6909Error> {
        if from == Address::ZERO {
            return Err(ERC6909Error::TransferFromZeroAddress(TransferFromZeroAddress {}));
        }
        
        if to == Address::ZERO {
            return Err(ERC6909Error::TransferToZeroAddress(TransferToZeroAddress {}));
        }

        let caller = msg::sender();
        
        // Check if caller is authorized to transfer
        if caller != from && !self.is_operator(from, caller) {
            let current_allowance = self.allowance(from, caller, id);
            if current_allowance < amount {
                return Err(ERC6909Error::InsufficientAllowance(InsufficientAllowance {}));
            }
            
            // Update allowance if not max value (infinite approval)
            if current_allowance != U256::MAX {
                let new_allowance = current_allowance - amount;
                self.allowances.setter(from).setter(caller).setter(id).set(new_allowance);
            }
        }

        // Check balance
        let from_balance = self.balance_of(from, id);
        if from_balance < amount {
            return Err(ERC6909Error::InsufficientBalance(InsufficientBalance {}));
        }

        // Update balances
        let new_from_balance = from_balance - amount;
        self.balances.setter(from).setter(id).set(new_from_balance);
        
        let to_balance = self.balance_of(to, id);
        let new_to_balance = to_balance + amount;
        self.balances.setter(to).setter(id).set(new_to_balance);

        // Emit transfer event using raw log
        evm::raw_log(&[
            // TransferSingle event signature
            alloy_primitives::keccak256("TransferSingle(address,address,address,uint256,uint256)").into(),
            caller.into_word().into(),
            from.into_word().into(),
            to.into_word().into(),
        ], &[id, amount].abi_encode()).ok();

        Ok(true)
    }

    /// Transfers tokens from caller to another address
    pub fn transfer(&mut self, to: Address, id: U256, amount: U256) -> Result<bool, ERC6909Error> {
        let from = msg::sender();
        self.transfer_from(from, to, id, amount)
    }

    /// Approves a spender to transfer a specific amount of tokens for a specific token ID
    pub fn approve(
        &mut self,
        spender: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, ERC6909Error> {
        let owner = msg::sender();
        
        self.allowances.setter(owner).setter(spender).setter(id).set(amount);

        // Emit approval event using raw log
        evm::raw_log(&[
            // ApprovalSingle event signature
            alloy_primitives::keccak256("ApprovalSingle(address,address,uint256,uint256)").into(),
            owner.into_word().into(),
            spender.into_word().into(),
            id.into(),
        ], &[amount].abi_encode()).ok();

        Ok(true)
    }

    /// Sets or unsets an operator for all tokens of the caller
    pub fn set_operator(&mut self, operator: Address, approved: bool) -> Result<bool, ERC6909Error> {
        let owner = msg::sender();
        
        if owner == operator {
            return Err(ERC6909Error::InvalidOperator(InvalidOperator {}));
        }

        self.operators.setter(owner).setter(operator).set(approved);

        // Emit operator event using raw log
        let approved_data = if approved { U256::from(1) } else { U256::from(0) };
        evm::raw_log(&[
            // OperatorSet event signature
            alloy_primitives::keccak256("OperatorSet(address,address,bool)").into(),
            owner.into_word().into(),
            operator.into_word().into(),
        ], &[approved_data].abi_encode()).ok();

        Ok(true)
    }

    /// Mints new tokens 
    pub fn mint(&mut self, to: Address, id: U256, amount: U256) -> Result<bool, ERC6909Error> {
        if to == Address::ZERO {
            return Err(ERC6909Error::TransferToZeroAddress(TransferToZeroAddress {}));
        }

        // Update balance
        let current_balance = self.balance_of(to, id);
        let new_balance = current_balance + amount;
        self.balances.setter(to).setter(id).set(new_balance);

        // Update total supply
        let current_supply = self.total_supply(id);
        let new_supply = current_supply + amount;
        self.total_supplies.setter(id).set(new_supply);

        // Emit transfer event from zero address using raw log
        let caller = msg::sender();
        evm::raw_log(&[
            // TransferSingle event signature
            alloy_primitives::keccak256("TransferSingle(address,address,address,uint256,uint256)").into(),
            caller.into_word().into(),
            Address::ZERO.into_word().into(),
            to.into_word().into(),
        ], &[id, amount].abi_encode()).ok();

        Ok(true)
    }

    /// Burns tokens 
    pub fn burn(&mut self, from: Address, id: U256, amount: U256) -> Result<bool, ERC6909Error> {
        let caller = msg::sender();
        
        // Check if caller is authorized
        if caller != from && !self.is_operator(from, caller) {
            let current_allowance = self.allowance(from, caller, id);
            if current_allowance < amount {
                return Err(ERC6909Error::InsufficientAllowance(InsufficientAllowance {}));
            }
            
            if current_allowance != U256::MAX {
                let new_allowance = current_allowance - amount;
                self.allowances.setter(from).setter(caller).setter(id).set(new_allowance);
            }
        }

        // Check balance
        let from_balance = self.balance_of(from, id);
        if from_balance < amount {
            return Err(ERC6909Error::InsufficientBalance(InsufficientBalance {}));
        }

        // Update balance
        let new_balance = from_balance - amount;
        self.balances.setter(from).setter(id).set(new_balance);

        // Update total supply
        let current_supply = self.total_supply(id);
        let new_supply = current_supply - amount;
        self.total_supplies.setter(id).set(new_supply);

        // Emit transfer event to zero address using raw log
        evm::raw_log(&[
            // TransferSingle event signature
            alloy_primitives::keccak256("TransferSingle(address,address,address,uint256,uint256)").into(),
            caller.into_word().into(),
            from.into_word().into(),
            Address::ZERO.into_word().into(),
        ], &[id, amount].abi_encode()).ok();

        Ok(true)
    }

    /// Batch transfer multiple token types
    pub fn batch_transfer_from(
        &mut self,
        from: Address,
        to: Address,
        ids: alloc::vec::Vec<U256>,
        amounts: alloc::vec::Vec<U256>,
    ) -> Result<bool, ERC6909Error> {
        if ids.len() != amounts.len() {
            return Err(ERC6909Error::InvalidOperator(InvalidOperator {}));
        }

        for (id, amount) in ids.iter().zip(amounts.iter()) {
            self.transfer_from(from, to, *id, *amount)?;
        }

        Ok(true)
    }
}
