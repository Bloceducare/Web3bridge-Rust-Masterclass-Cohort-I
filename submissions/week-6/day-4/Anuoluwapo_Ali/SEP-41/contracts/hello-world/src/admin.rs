use soroban_sdk::{Address, Env};
use crate::storage::{set_admin, get_admin, has_admin};

pub fn require_admin(env: &Env, addr: &Address) {
    let admin = get_admin(env);
    if *addr != admin {
        panic!("Unauthorized: Only admin can perform this action");
    }
}

pub fn change_admin(env: &Env, current_admin: &Address, new_admin: &Address) {
    current_admin.require_auth();
    require_admin(env, current_admin);
    set_admin(env, new_admin);
}