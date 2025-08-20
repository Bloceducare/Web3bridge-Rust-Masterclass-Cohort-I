#[cfg(test)]
mod test {
    use crate::token::{Sep41Token, Sep41TokenClient};
    use soroban_sdk::{Address, Env, String, testutils::{Address as _, Ledger}};

    fn create_contract<'a>() -> (Env, Sep41TokenClient<'a>, Address) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(Sep41Token, ());
        let client = Sep41TokenClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        (env, client, admin)
    }

    #[test]
    fn test_initialize() {
        let (env, client, admin) = create_contract();
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        assert_eq!(client.name(), String::from_str(&env, "MyToken"));
        assert_eq!(client.symbol(), String::from_str(&env, "MTK"));
        assert_eq!(client.decimals(), 7);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_initialize_twice() {
        let (env, client, admin) = create_contract();
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
    }

    #[test]
    fn test_mint_and_balance() {
        let (env, client, admin) = create_contract();
        let user = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user, &1000);
        assert_eq!(client.balance(&user), 1000);
    }

    #[test]
    fn test_transfer() {
        let (env, client, admin) = create_contract();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user1, &1000);
        client.transfer(&user1, &user2, &500);
        assert_eq!(client.balance(&user1), 500);
        assert_eq!(client.balance(&user2), 500);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_transfer_insufficient_balance() {
        let (env, client, admin) = create_contract();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user1, &100);
        client.transfer(&user1, &user2, &200);
    }

    #[test]
    fn test_approve_and_transfer_from() {
        let (env, client, admin) = create_contract();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        let spender = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user1, &1000);
        client.approve(&user1, &spender, &500, &100);
        assert_eq!(client.allowance(&user1, &spender), 500);
        client.transfer_from(&spender, &user1, &user2, &300);
        assert_eq!(client.balance(&user1), 700);
        assert_eq!(client.balance(&user2), 300);
        assert_eq!(client.allowance(&user1, &spender), 200);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_transfer_from_insufficient_allowance() {
        let (env, client, admin) = create_contract();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        let spender = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user1, &1000);
        client.approve(&user1, &spender, &100, &100);
        client.transfer_from(&spender, &user1, &user2, &200);
    }

    #[test]
    fn test_burn() {
        let (env, client, admin) = create_contract();
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&admin, &1000);
        client.burn(&admin, &500);
        assert_eq!(client.balance(&admin), 500);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #5)")]
    fn test_burn_unauthorized() {
        let (env, client, admin) = create_contract();
        let user = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user, &1000);
        client.burn(&user, &500);
    }

    #[test]
    fn test_burn_from() {
        let (env, client, admin) = create_contract();
        let user = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user, &1000);
        client.approve(&user, &admin, &500, &100);
        client.burn_from(&admin, &user, &300);
        assert_eq!(client.balance(&user), 700);
        assert_eq!(client.allowance(&user, &admin), 200);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_burn_insufficient_balance() {
        let (env, client, admin) = create_contract();
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&admin, &100);
        client.burn(&admin, &200);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #4)")]
    fn test_burn_from_insufficient_allowance() {
        let (env, client, admin) = create_contract();
        let user = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user, &1000);
        client.approve(&user, &admin, &100, &100);
        client.burn_from(&admin, &user, &200);
    }

    #[test]
    fn test_multiple_transfers() {
        let (env, client, admin) = create_contract();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        let user3 = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.mint(&user1, &1000);
        client.transfer(&user1, &user2, &300);
        client.transfer(&user2, &user3, &100);
        assert_eq!(client.balance(&user1), 700);
        assert_eq!(client.balance(&user2), 200);
        assert_eq!(client.balance(&user3), 100);
    }

    #[test]
    fn test_allowance_expiration() {
        let (env, client, admin) = create_contract();
        let user = Address::generate(&env);
        let spender = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.approve(&user, &spender, &500, &100);
        env.ledger().with_mut(|li| li.sequence_number = 101);
        assert_eq!(client.allowance(&user, &spender), 0);
    }

    #[test]
    fn test_approve_expiry_edge_case() {
        let (env, client, admin) = create_contract();
        let user = Address::generate(&env);
        let spender = Address::generate(&env);
        client.initialize(&admin, &7, &String::from_str(&env, "MyToken"), &String::from_str(&env, "MTK"));
        client.approve(&user, &spender, &500, &50);
        env.ledger().with_mut(|li| li.sequence_number = 50);
        assert_eq!(client.allowance(&user, &spender), 500);
        env.ledger().with_mut(|li| li.sequence_number = 51);
        assert_eq!(client.allowance(&user, &spender), 0);
    }
}