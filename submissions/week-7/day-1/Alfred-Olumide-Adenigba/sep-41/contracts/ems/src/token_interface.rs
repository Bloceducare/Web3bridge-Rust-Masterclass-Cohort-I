use soroban_sdk::{Address, Env, contractclient};

#[contractclient(name = "TokenClient")]
pub trait TokenInterface {
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn balance_of(env: Env, owner: Address) -> i128;
    fn initialize(env: Env, admin: Address, total_supply: i128);
}