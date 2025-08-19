use soroban_sdk::{Address, Env};
use crate::storage::get_admin;

pub trait AdminCheck {
    fn require_admin(&self, env: &Env);
}

impl AdminCheck for Address {
    fn require_admin(&self, env: &Env) {
        let admin = get_admin(env);
        if *self != admin {
            panic!("Only admin can call this function");
        }
    }
}

pub trait TokenValidation {
    fn validate_positive_amount(&self);
    fn validate_non_zero(&self);
}

impl TokenValidation for i128 {
    fn validate_positive_amount(&self) {
        if *self < 0 {
            panic!("Amount must be non-negative");
        }
    }

    fn validate_non_zero(&self) {
        if *self == 0 {
            panic!("Amount must be greater than zero");
        }
    }
}
