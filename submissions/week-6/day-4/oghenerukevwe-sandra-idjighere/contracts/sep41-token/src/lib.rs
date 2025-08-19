#![no_std]

//! # SEP-41 Token Contract
//! 
//! This contract implements the SEP-41 token interface for Stellar/Soroban,
//! providing a standard fungible token with the following features:
//! 
//! - Transfer tokens between addresses
//! - Approve spending allowances with expiration
//! - Burn tokens to reduce supply
//! - Administrative functions for minting and management
//! - Full compliance with SEP-41 specification
//! 
//! ## Usage
//! 
//! 1. Initialize the contract with metadata and admin
//! 2. Mint initial supply to desired addresses
//! 3. Users can transfer, approve, and burn tokens
//! 
//! ## Security Features
//! 
//! - Authorization required for all state-changing operations
//! - Allowances have expiration to prevent stale approvals
//! - Input validation on all parameters
//! - Proper event emission for transparency

mod contract;
mod interface;
mod storage;

pub use contract::Token;
pub use interface::{TokenInterface, TokenAdminInterface};

#[cfg(test)]
mod test_simple;

// Re-export the contract for external use
pub use contract::Token as Sep41Token;
