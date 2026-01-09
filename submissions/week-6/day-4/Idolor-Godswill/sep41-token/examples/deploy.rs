use sep41_token::{Token, TokenInterface, TokenAdminInterface};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

/// Example deployment and usage of the SEP-41 token
fn main() {
    println!("SEP-41 Token Deployment Example");
    
    // This example shows how to deploy and use the token
    // In a real deployment, you would use the Stellar CLI
    
    let env = Env::default();
    env.mock_all_auths();
    
    // Generate addresses
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    println!("Admin address: {:?}", admin);
    println!("User1 address: {:?}", user1);
    println!("User2 address: {:?}", user2);
    
    // Step 1: Initialize the token
    println!("\n=== Initializing Token ===");
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Godswill Token"),
        String::from_str(&env, "GWT"),
        18,
    );
    
    println!("Token Name: {}", Token::name(env.clone()));
    println!("Token Symbol: {}", Token::symbol(env.clone()));
    println!("Token Decimals: {}", Token::decimals(env.clone()));
    println!("Token Admin: {:?}", Token::admin(env.clone()));
    
    // Step 2: Mint initial supply
    println!("\n=== Minting Initial Supply ===");
    let initial_supply = 1_000_000_000_000_000_000i128; // 1M tokens with 18 decimals
    Token::mint(env.clone(), admin.clone(), initial_supply);
    
    println!("Admin balance after minting: {}", Token::balance(env.clone(), admin.clone()));
    
    // Step 3: Transfer tokens
    println!("\n=== Transferring Tokens ===");
    let transfer_amount = 100_000_000_000_000_000i128; // 100K tokens
    Token::transfer(env.clone(), admin.clone(), user1.clone(), transfer_amount);
    
    println!("Admin balance after transfer: {}", Token::balance(env.clone(), admin.clone()));
    println!("User1 balance after transfer: {}", Token::balance(env.clone(), user1.clone()));
    
    // Step 4: Approve and transfer from
    println!("\n=== Approval and Transfer From ===");
    let allowance_amount = 50_000_000_000_000_000i128; // 50K tokens
    let expiration = env.ledger().sequence() + 1000; // Expires in 1000 ledgers
    
    Token::approve(env.clone(), user1.clone(), user2.clone(), allowance_amount, expiration);
    println!("Allowance set: {}", Token::allowance(env.clone(), user1.clone(), user2.clone()));
    
    let transfer_from_amount = 25_000_000_000_000_000i128; // 25K tokens
    Token::transfer_from(env.clone(), user2.clone(), user1.clone(), admin.clone(), transfer_from_amount);
    
    println!("User1 balance after transfer_from: {}", Token::balance(env.clone(), user1.clone()));
    println!("Admin balance after receiving transfer_from: {}", Token::balance(env.clone(), admin.clone()));
    println!("Remaining allowance: {}", Token::allowance(env.clone(), user1.clone(), user2.clone()));
    
    // Step 5: Burn tokens
    println!("\n=== Burning Tokens ===");
    let burn_amount = 10_000_000_000_000_000i128; // 10K tokens
    Token::burn(env.clone(), user1.clone(), burn_amount);
    
    println!("User1 balance after burning: {}", Token::balance(env.clone(), user1.clone()));
    
    // Step 6: Burn from (using allowance)
    println!("\n=== Burn From (Using Allowance) ===");
    let burn_from_amount = 5_000_000_000_000_000i128; // 5K tokens
    Token::burn_from(env.clone(), user2.clone(), user1.clone(), burn_from_amount);
    
    println!("User1 balance after burn_from: {}", Token::balance(env.clone(), user1.clone()));
    println!("Final allowance: {}", Token::allowance(env.clone(), user1.clone(), user2.clone()));
    
    println!("\n=== Final Balances ===");
    println!("Admin: {}", Token::balance(env.clone(), admin));
    println!("User1: {}", Token::balance(env.clone(), user1));
    println!("User2: {}", Token::balance(env, user2));
    
    println!("\n✅ SEP-41 Token deployment and usage example completed successfully!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_example() {
        // This test ensures the deployment example works correctly
        main();
    }
}
