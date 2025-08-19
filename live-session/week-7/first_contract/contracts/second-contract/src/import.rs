pub mod contract_a {
    use soroban_sdk::contractimport;
    contractimport!(file = "../../target/wasm32v1-none/release/first_contract.wasm");
}
