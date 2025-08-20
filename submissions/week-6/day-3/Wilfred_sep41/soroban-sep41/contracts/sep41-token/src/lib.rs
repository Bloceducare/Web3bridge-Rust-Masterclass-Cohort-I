
#![no_std]

mod contract;
mod interface;
mod storage;
mod events;
mod metadata;
mod admin;
mod error;

// Re-export the main contract and interfaces
pub use contract::Token;
pub use interface::{TokenInterface, TokenAdminInterface};
pub use error::TokenError;

// Re-export commonly used types
pub use storage::{DataKey, AllowanceDataKey, AllowanceValue, TokenMetadata};
pub use events::{TransferEvent, ApprovalEvent, BurnEvent, MintEvent};

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_initialization() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 18;

        Token::initialize(env.clone(), admin.clone(), name.clone(), symbol.clone(), decimals);

        assert_eq!(Token::name(env.clone()), name);
        assert_eq!(Token::symbol(env.clone()), symbol);
        assert_eq!(Token::decimals(env.clone()), decimals);
        assert_eq!(Token::admin(env), admin);
    }

    #[test]
    fn test_mint_and_balance() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let amount = 1000i128;

        // Initialize token
        Token::initialize(
            env.clone(),
            admin.clone(),
            String::from_str(&env, "Test Token"),
            String::from_str(&env, "TEST"),
            18,
        );

        // Mint tokens
        Token::mint(env.clone(), user.clone(), amount);

        // Check balance
        assert_eq!(Token::balance(env, user), amount);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        let amount = 1000i128;
        let transfer_amount = 300i128;

        // Initialize and mint
        Token::initialize(
            env.clone(),
            admin.clone(),
            String::from_str(&env, "Test Token"),
            String::from_str(&env, "TEST"),
            18,
        );
        Token::mint(env.clone(), user1.clone(), amount);

        // Transfer
        Token::transfer(env.clone(), user1.clone(), user2.clone(), transfer_amount);

        // Check balances
        assert_eq!(Token::balance(env.clone(), user1), amount - transfer_amount);
        assert_eq!(Token::balance(env, user2), transfer_amount);
    }

    #[test]
    fn test_approve_and_transfer_from() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);
        let amount = 1000i128;
        let allowance_amount = 500i128;
        let transfer_amount = 300i128;
        let expiration = env.ledger().sequence() + 100;

        // Initialize and mint
        Token::initialize(
            env.clone(),
            admin.clone(),
            String::from_str(&env, "Test Token"),
            String::from_str(&env, "TEST"),
            18,
        );
        Token::mint(env.clone(), owner.clone(), amount);

        // Approve
        Token::approve(env.clone(), owner.clone(), spender.clone(), allowance_amount, expiration);
        assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), allowance_amount);

        // Transfer from
        Token::transfer_from(env.clone(), spender.clone(), owner.clone(), recipient.clone(), transfer_amount);

        // Check balances and remaining allowance
        assert_eq!(Token::balance(env.clone(), owner), amount - transfer_amount);
        assert_eq!(Token::balance(env.clone(), recipient), transfer_amount);
        assert_eq!(Token::allowance(env, owner, spender), allowance_amount - transfer_amount);
    }

    #[test]
    fn test_burn() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let amount = 1000i128;
        let burn_amount = 300i128;

        // Initialize and mint
        Token::initialize(
            env.clone(),
            admin.clone(),
            String::from_str(&env, "Test Token"),
            String::from_str(&env, "TEST"),
            18,
        );
        Token::mint(env.clone(), user.clone(), amount);

        // Burn
        Token::burn(env.clone(), user.clone(), burn_amount);

        // Check balance
        assert_eq!(Token::balance(env, user), amount - burn_amount);
    }
}
