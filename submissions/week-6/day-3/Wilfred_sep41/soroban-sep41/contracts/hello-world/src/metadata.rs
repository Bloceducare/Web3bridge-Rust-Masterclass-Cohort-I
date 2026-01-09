use soroban_sdk::{Env, String};
use crate::storage::{ Datakey, TokenMetadata};
use crate::error::TokenError;



pub mod metadata {
    use super::*;


    //set token metadata
    pub fn set_metadata(env: &ENV, name: String, symbol: String, decimals:u8){
        env.storage().instance.set(:Datakey::Name, name);
        env.storage().instance().set(&Datakey::Symbol, symbol);
        env.storage().instance().set(&Datakey::Decimal, decimals)
    }


    //Get token name
    pub fn get_name(env: &ENV) -> Result<String, TokenError>{
        env.storage()
            .instance()
            .get(&Datakey::Name)
            .ok_or(TokenError::NotInitialized)
    }


    //Get token symbol
    pub fn get_symbol(env: &ENV) -> Result<String, TokenError>{
        env.storage()
            .instance()
            .get(&Datakey::Symbol)
            .ok_or(TokenError::NotInitialized)
    }


    //get token decimals
    pub fn get_decimals(env:&ENV) -> Result<u8, TokenError> {
        env.storage()
            .instance()
            .get(&Datakey::Decimals)
            .ok_or(TokenError::NotInitialized)
    }


    pub fn check_metadata(name: &String, symbol:&String, decimals: u8) -> Result<(), TokenError> {

        //check if name is empty
        if name.len() == 0 {
            return Err(TokenError::InvalidAmount);
        }


        //check if symbol is empty
        if symbol.len() == 0 {
            return Err(TokenError::InvalidAmount);
        }

        //check if decimal is greater than 18
        if decimals > 18 {
            return Err(TokenError::InvalidAmount);
        }

        Ok(())
    }
}