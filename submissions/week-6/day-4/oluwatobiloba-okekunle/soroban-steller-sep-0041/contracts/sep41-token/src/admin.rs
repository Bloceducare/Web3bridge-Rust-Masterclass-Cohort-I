use soroban_sdk::{Address, Env};

use crate::{error::TokenError, events, storage};

pub fn has_administrator(e: &Env) -> bool {
    storage::has_admin(e)
}

pub fn read_administrator(e: &Env) -> Address {
    storage::get_admin(e)
}

pub fn write_administrator(e: &Env, id: &Address) {
    storage::set_admin(e, id);
}

pub fn check_admin(e: &Env, auth: &Address) -> Result<(), TokenError> {
    // Add initialization check
    if !has_administrator(e) {
        return Err(TokenError::NotInitialized);
    }

    let admin = storage::get_admin(e);
    if *auth == admin {
        Ok(())
    } else {
        Err(TokenError::Unauthorized)
    }
}

pub fn set_admin(e: &Env, new_admin: &Address) -> Result<(), TokenError> {
    // Check if admin exists before trying to read it
    if !has_administrator(e) {
        return Err(TokenError::NotInitialized);
    }

    let admin = read_administrator(e);
    admin.require_auth();

    check_admin(e, &admin)?;
    write_administrator(e, new_admin);
    events::set_admin_event(e, admin, new_admin.clone());
    storage::extend_instance(e);

    Ok(())
}
