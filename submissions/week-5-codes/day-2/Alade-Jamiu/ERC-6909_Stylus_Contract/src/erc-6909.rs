// SPDX-License-Identifier: MIT
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    contract, evm, msg, storage::{StorageMap, StorageBool},
    prelude::*,
};

// Define the contract
#[solidity_storage]
#[entrypoint]
struct ERC6909 {
    // Total supply per token ID
    total_supply: StorageMap<U256, U256>,
    // Balances: owner -> token_id -> amount
    balances: StorageMap<Address, StorageMap<U256, U256>>,
    // Allowances: owner -> spender -> token_id -> amount
    allowances: StorageMap<Address, StorageMap<Address, StorageMap<U256, U256>>>,
    // Operator approvals: owner -> operator -> bool
    operators: StorageMap<Address, StorageMap<Address, StorageBool>>,
}

// Define events
#[event]
struct TransferSingle {
    #[indexed]
    operator: Address,
    #[indexed]
    from: Address,
    #[indexed]
    to: Address,
    token_id: U256,
    amount: U256,
}

#[event]
struct ApprovalSingle {
    #[indexed]
    owner: Address,
    #[indexed]
    spender: Address,
    token_id: U256,
    amount: U256,
}

#[event]
struct OperatorSet {
    #[indexed]
    owner: Address,
    #[indexed]
    operator: Address,
    approved: bool,
}

// Define custom errors
#[error]
enum ERC6909Error {
    InsufficientBalance,
    InsufficientAllowance,
    ZeroAddress,
}

// Implement the contract
#[external]
impl ERC6909 {
    // Query total supply for a token ID
    #[view]
    fn total_supply(&self, token_id: U256) -> U256 {
        self.total_supply.get(token_id)
    }

    // Query balance of an owner for a token ID
    #[view]
    fn balance_of(&self, owner: Address, token_id: U256) -> U256 {
        if owner.is_zero() {
            panic_with_error!(ERC6909Error::ZeroAddress);
        }
        self.balances.get(owner).get(token_id)
    }

    // Transfer tokens from sender to recipient
    fn transfer(&mut self, to: Address, token_id: U256, amount: U256) -> bool {
        self.transfer_from(msg::sender(), to, token_id, amount)
    }

    // Transfer tokens from one address to another
    fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        token_id: U256,
        amount: U256,
    ) -> bool {
        if from.is_zero() || to.is_zero() {
            panic_with_error!(ERC6909Error::ZeroAddress);
        }

        let sender = msg::sender();
        let is_operator = self.operators.get(from).get(sender).get();
        
        // Check if sender is authorized (either from or approved operator)
        if sender != from && !is_operator {
            let allowance = self.allowances.get(from).get(sender).get(token_id);
            if allowance < amount {
                panic_with_error!(ERC6909Error::InsufficientAllowance);
            }
            // Decrease allowance
            self.allowances
                .get(from)
                .get(sender)
                .insert(token_id, allowance - amount);
        }

        // Check balance
        let balance = self.balances.get(from).get(token_id);
        if balance < amount {
            panic_with_error!(ERC6909Error::InsufficientBalance);
        }

        // Update balances
        self.balances.get(from).insert(token_id, balance - amount);
        self.balances.get(to).insert(token_id, self.balances.get(to).get(token_id) + amount);

        // Emit TransferSingle event
        evm::log(TransferSingle {
            operator: sender,
            from,
            to,
            token_id,
            amount,
        });

        true
    }

    // Approve a spender for a specific token ID and amount
    fn approve(&mut self, spender: Address, token_id: U256, amount: U256) -> bool {
        if spender.is_zero() {
            panic_with_error!(ERC6909Error::ZeroAddress);
        }

        let owner = msg::sender();
        self.allowances.get(owner).get(spender).insert(token_id, amount);

        // Emit ApprovalSingle event
        evm::log(ApprovalSingle {
            owner,
            spender,
            token_id,
            amount,
        });

        true
    }

    // Set operator approval for all tokens of an owner
    fn set_operator(&mut self, operator: Address, approved: bool) -> bool {
        if operator.is_zero() {
            panic_with_error!(ERC6909Error::ZeroAddress);
        }

        let owner = msg::sender();
        self.operators.get(owner).get(operator).set(approved);

        // Emit OperatorSet event
        evm::log(OperatorSet {
            owner,
            operator,
            approved,
        });

        true
    }

    // Query operator approval status
    #[view]
    fn operator_approval(&self, owner: Address, operator: Address) -> bool {
        if owner.is_zero() || operator.is_zero() {
            panic_with_error!(ERC6909Error::ZeroAddress);
        }
        self.operators.get(owner).get(operator).get()
    }

    // Optional: Mint function for testing (not part of ERC-6909 standard)
    fn mint(&mut self, to: Address, token_id: U256, amount: U256) {
        if to.is_zero() {
            panic_with_error!(ERC6909Error::ZeroAddress);
        }

        self.total_supply.insert(token_id, self.total_supply.get(token_id) + amount);
        self.balances.get(to).insert(token_id, self.balances.get(to).get(token_id) + amount);

        evm::log(TransferSingle {
            operator: msg::sender(),
            from: Address::zero(),
            to,
            token_id,
            amount,
        });
    }
}