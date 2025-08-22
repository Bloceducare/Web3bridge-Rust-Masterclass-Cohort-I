use child::child::ChildContractClient;

use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, Vec};

#[contracttype]
pub enum DataKey {
    Children,
    UserChildren(Address),
}
#[contract]
pub struct FactoryContract;

#[contractimpl]
impl FactoryContract {
    pub fn create_child(
        env: Env,
        a: i32,
        b: i32,
        owner: Address,
        salt: BytesN<32>,
        wasm_hash: BytesN<32>,
    ) {
        let child_address = env
            .deployer()
            .with_address(owner.clone(), salt)
            .deploy_v2(wasm_hash, ());

        let mut get_deployed_add = Self::get_user_deployed_addresses(&env, owner.clone());

        let mut children = Self::get_all_children(&env);

        children.push_back(child_address.clone());

        let child = ChildContractClient::new(&env, &child_address);

        get_deployed_add.push_back(child_address.clone());

        env.storage()
            .persistent()
            .set(&DataKey::UserChildren(owner), &child_address.clone());

        child.sub(&a, &b);
    }

    pub fn get_user_deployed_addresses(env: &Env, owner: Address) -> Vec<Address> {
        env.storage()
            .persistent()
            .get(&DataKey::UserChildren(owner))
            .unwrap_or(Vec::new(env))
    }

    pub fn get_all_children(env: &Env) -> Vec<Address> {
        env.storage()
            .persistent()
            .get(&DataKey::Children)
            .unwrap_or(Vec::new(env))
    }
}
