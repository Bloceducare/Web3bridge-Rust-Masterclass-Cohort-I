#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    alloy_sol_types::{sol, SolError},
    prelude::*,
    storage::{StorageBool, StorageMap, StorageU256},
};

// ERC-6909 Events
sol! {
    event TransferSingle(
        address indexed operator,
        address indexed from,
        address indexed to,
        uint256 id,
        uint256 amount
    );
    
    event ApprovalSingle(
        address indexed owner,
        address indexed spender,
        uint256 indexed id,
        uint256 amount
    );
    
    event OperatorSet(
        address indexed owner,
        address indexed operator,
        bool approved
    );
}

// Custom errors
sol! {
    error InsufficientBalance();
    error InsufficientAllowance();
    error InvalidReceiver();
    error Unauthorized();
}

// Storage structure for the ERC-6909 contract
#[storage]
#[entrypoint]
pub struct ERC6909Contract {
    /// Balances mapping: owner -> token_id -> balance
    balances: StorageMap<Address, StorageMap<U256, StorageU256>>,
    
    /// Allowances mapping: owner -> spender -> token_id -> amount
    allowances: StorageMap<Address, StorageMap<Address, StorageMap<U256, StorageU256>>>,
    
    /// Operator approvals: owner -> operator -> approved
    operators: StorageMap<Address, StorageMap<Address, StorageBool>>,
    
    /// Total supply per token ID
    total_supplies: StorageMap<U256, StorageU256>,
}

#[public]
impl ERC6909Contract {
    /// Returns the total supply of a specific token ID
    pub fn total_supply(&self, token_id: U256) -> U256 {
        self.total_supplies.get(token_id)
    }
    
    /// Returns the balance of an owner for a specific token ID
    pub fn balance_of(&self, owner: Address, token_id: U256) -> U256 {
        self.balances.get(owner).get(token_id)
    }
    
    /// Returns the allowance amount for a spender on a specific token ID
    pub fn allowance(&self, owner: Address, spender: Address, token_id: U256) -> U256 {
        self.allowances.get(owner).get(spender).get(token_id)
    }
    
    /// Returns whether an operator is approved for all tokens of an owner
    pub fn is_operator(&self, owner: Address, operator: Address) -> bool {
        self.operators.get(owner).get(operator)
    }
    
    /// Transfers tokens from one address to another
    /// Can be called by the owner, approved spender, or approved operator
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        token_id: U256,
        amount: U256,
    ) -> Result<bool, Vec<u8>> {
        if to == Address::ZERO {
            return Err(InvalidReceiver {}.abi_encode());
        }
        
        let caller = self.vm().msg_sender();
        
        // Check authorization
        if caller != from && !self._is_authorized(from, caller, token_id, amount) {
            return Err(Unauthorized {}.abi_encode());
        }
        
        // Check sufficient balance
        let from_balance = self.balance_of(from, token_id);
        if from_balance < amount {
            return Err(InsufficientBalance {}.abi_encode());
        }
        
        // Update balances
        let mut from_balances = self.balances.setter(from);
        let mut from_balance_storage = from_balances.setter(token_id);
        from_balance_storage.set(from_balance - amount);
        
        let to_balance = self.balance_of(to, token_id);
        let mut to_balances = self.balances.setter(to);
        let mut to_balance_storage = to_balances.setter(token_id);
        to_balance_storage.set(to_balance + amount);
        
        // Update allowance if caller is not owner and not operator
        if caller != from && !self.is_operator(from, caller) {
            let current_allowance = self.allowance(from, caller, token_id);
            let mut allowances_for_owner = self.allowances.setter(from);
            let mut allowances_for_spender = allowances_for_owner.setter(caller);
            let mut allowance_storage = allowances_for_spender.setter(token_id);
            allowance_storage.set(current_allowance - amount);
        }
        
        // Emit event
        log(self.vm(), TransferSingle {
            operator: caller,
            from,
            to,
            id: token_id,
            amount,
        });
        
        Ok(true)
    }
    
    /// Approves a spender to spend a specific amount of a token ID
    pub fn approve(
        &mut self,
        spender: Address,
        token_id: U256,
        amount: U256,
    ) -> Result<bool, Vec<u8>> {
        let owner = self.vm().msg_sender();
        
        let mut allowances_for_owner = self.allowances.setter(owner);
        let mut allowances_for_spender = allowances_for_owner.setter(spender);
        let mut allowance_storage = allowances_for_spender.setter(token_id);
        allowance_storage.set(amount);
        
        log(self.vm(), ApprovalSingle {
            owner,
            spender,
            id: token_id,
            amount,
        });
        
        Ok(true)
    }
    
    /// Sets or unsets an operator for all tokens of the caller
    pub fn set_operator(&mut self, operator: Address, approved: bool) -> Result<bool, Vec<u8>> {
        let owner = self.vm().msg_sender();
        
        let mut operators_for_owner = self.operators.setter(owner);
        let mut operator_storage = operators_for_owner.setter(operator);
        operator_storage.set(approved);
        
        log(self.vm(), OperatorSet {
            owner,
            operator,
            approved,
        });
        
        Ok(true)
    }
    
    /// Mints new tokens (only for demonstration - in production, add access control)
    pub fn mint(&mut self, to: Address, token_id: U256, amount: U256) -> Result<bool, Vec<u8>> {
        if to == Address::ZERO {
            return Err(InvalidReceiver {}.abi_encode());
        }
        
        // Update balance
        let current_balance = self.balance_of(to, token_id);
        let mut to_balances = self.balances.setter(to);
        let mut balance_storage = to_balances.setter(token_id);
        balance_storage.set(current_balance + amount);
        
        // Update total supply
        let current_supply = self.total_supply(token_id);
        let mut supply_storage = self.total_supplies.setter(token_id);
        supply_storage.set(current_supply + amount);
        
        // Emit event
        log(self.vm(), TransferSingle {
            operator: self.vm().msg_sender(),
            from: Address::ZERO,
            to,
            id: token_id,
            amount,
        });
        
        Ok(true)
    }
    
    /// Burns tokens (only for demonstration - in production, add access control)
    pub fn burn(&mut self, from: Address, token_id: U256, amount: U256) -> Result<bool, Vec<u8>> {
        let current_balance = self.balance_of(from, token_id);
        if current_balance < amount {
            return Err(InsufficientBalance {}.abi_encode());
        }
        
        // Update balance
        let mut from_balances = self.balances.setter(from);
        let mut balance_storage = from_balances.setter(token_id);
        balance_storage.set(current_balance - amount);
        
        // Update total supply
        let current_supply = self.total_supply(token_id);
        let mut supply_storage = self.total_supplies.setter(token_id);
        supply_storage.set(current_supply - amount);
        
        // Emit event
        log(self.vm(), TransferSingle {
            operator: self.vm().msg_sender(),
            from,
            to: Address::ZERO,
            id: token_id,
            amount,
        });
        
        Ok(true)
    }
}

impl ERC6909Contract {
    /// Internal helper to check if caller is authorized to transfer tokens
    fn _is_authorized(&self, owner: Address, caller: Address, token_id: U256, amount: U256) -> bool {
        // Check if caller is an approved operator
        if self.is_operator(owner, caller) {
            return true;
        }
        
        // Check if caller has sufficient allowance
        let allowance = self.allowance(owner, caller, token_id);
        allowance >= amount
    }
}
