use soroban_sdk::{Address, Env, panic_with_error};

use crate::storage::ADMIN;
use crate::errors::TokenError;

pub trait AdminInterface {
    fn require_admin_auth(env: &Env) -> Address;
    fn is_admin(env: &Env, address: &Address) -> bool;
    fn check_admin_exists(env: &Env) -> bool;
}

pub struct AdminManager;

impl AdminInterface for AdminManager {
    fn require_admin_auth(env: &Env) -> Address {
        let admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic_with_error!(env, TokenError::NotInitialized));

        admin.require_auth();
        admin
    }

    fn is_admin(env: &Env, address: &Address) -> bool {
        if let Some(admin) = env.storage().instance().get::<_, Address>(&ADMIN) {
            admin == *address
        } else {
            false
        }
    }

    fn check_admin_exists(env: &Env) -> bool {
        env.storage().instance().has(&ADMIN)
    }
}

pub struct AdminOperations;

impl AdminOperations {
    pub fn transfer_admin_rights(env: &Env, current_admin: &Address, new_admin: &Address) {
        current_admin.require_auth();
        
        if !AdminManager::is_admin(env, current_admin) {
            panic_with_error!(env, TokenError::Unauthorized);
        }
        
        env.storage().instance().set(&ADMIN, new_admin);
    }

    pub fn revoke_admin(env: &Env, admin: &Address) {
        admin.require_auth();
        
        if !AdminManager::is_admin(env, admin) {
            panic_with_error!(env, TokenError::Unauthorized);
        }
        
        env.storage().instance().remove(&ADMIN);
    }
}