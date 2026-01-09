#[cfg(test)]

mod tests {
    use crate::sep41::{Sep41, Sep41Client};
    use soroban_sdk::String;
    use soroban_sdk::{testutils::Address as _, Address, Env};

    fn setup() -> (Env, Sep41Client<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, Sep41);
        let client = Sep41Client::new(&env, &contract_id);
        
        (env, client)
    }

    #[test]
    fn test_initialize() {
        let (env, client) = setup();
        let admin = Address::generate(&env);

        client.initialize(&admin);

        let stored_admin = client.admin();
        assert_eq!(stored_admin, admin);
    }

    #[test]
    fn test_metadata() {
        let (env, client) = setup();

        let decimals = client.decimals();
        let name = client.name();
        let symbol = client.symbol();

        assert_eq!(decimals, 18);
        assert_eq!(name, String::from_str(&env, "Age Devs"));
        assert_eq!(symbol, String::from_str(&env, "AGEDEVS"));
    }

    #[test]
    fn test_mint() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        client.initialize(&admin);
        client.mint(&user, &1000);

        let balance = client.balance(&user);
        assert_eq!(balance, 1000);
    }

    #[test]
    fn test_balance_empty() {
        let (env, client) = setup();
        let user = Address::generate(&env);

        let balance = client.balance(&user);
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_transfer() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let agedevs = Address::generate(&env);
        let gbolahan = Address::generate(&env);

        client.initialize(&admin);
        client.mint(&agedevs, &1000);

        client.transfer(&agedevs, &gbolahan, &300);

        let agedevs_balance = client.balance(&agedevs);
        let gbolahan_balance = client.balance(&gbolahan);

        assert_eq!(agedevs_balance, 700);
        assert_eq!(gbolahan_balance, 300);
    }

    #[test]
    fn test_approve() {
        let (env, client) = setup();
        let agedevs = Address::generate(&env);
        let gbolahan = Address::generate(&env);

        let future_ledger = env.ledger().sequence() + 100;
        client.approve(&agedevs, &gbolahan, &500, &future_ledger);

        let allowance = client.allowance(&agedevs, &gbolahan);
        assert_eq!(allowance, 500);
    }

    #[test]
    fn test_transfer_from() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let agedevs = Address::generate(&env);
        let gbolahan = Address::generate(&env);
        let charlie = Address::generate(&env);

        client.initialize(&admin);
        client.mint(&agedevs, &1000);

        let future_ledger = env.ledger().sequence() + 100;
        client.approve(&agedevs, &gbolahan, &500, &future_ledger);

        client.transfer_from(&gbolahan, &agedevs, &charlie, &200);

        let agedevs_balance = client.balance(&agedevs);
        let charlie_balance = client.balance(&charlie);
        let remaining_allowance = client.allowance(&agedevs, &gbolahan);

        assert_eq!(agedevs_balance, 800);
        assert_eq!(charlie_balance, 200);
        assert_eq!(remaining_allowance, 300);
    }

    #[test]
    fn test_burn() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let agedevs = Address::generate(&env);

        client.initialize(&admin);
        client.mint(&agedevs, &1000);

        client.burn(&agedevs, &300);

        let balance = client.balance(&agedevs);
        assert_eq!(balance, 700);
    }

    #[test]
    fn test_burn_from() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let agedevs = Address::generate(&env);
        let gbolahan = Address::generate(&env);

        client.initialize(&admin);
        client.mint(&agedevs, &1000);

        let future_ledger = env.ledger().sequence() + 100;
        client.approve(&agedevs, &gbolahan, &500, &future_ledger);

        client.burn_from(&gbolahan, &agedevs, &200);

        let agedevs_balance = client.balance(&agedevs);
        let remaining_allowance = client.allowance(&agedevs, &gbolahan);

        assert_eq!(agedevs_balance, 800);
        assert_eq!(remaining_allowance, 300);
    }

    #[test]
    fn test_set_admin() {
        let (env, client) = setup();
        let admin = Address::generate(&env);
        let new_admin = Address::generate(&env);

        client.initialize(&admin);
        client.set_admin(&new_admin);

        let stored_admin = client.admin();
        assert_eq!(stored_admin, new_admin);
    }
}
