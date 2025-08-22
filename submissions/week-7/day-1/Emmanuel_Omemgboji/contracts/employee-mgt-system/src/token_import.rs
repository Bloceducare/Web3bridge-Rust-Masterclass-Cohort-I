pub mod token_contract {
    use soroban_sdk::contractimport;
    contractimport!(file = "../../target/wasm32v1-none/release/ballor_token.wasm");
}
