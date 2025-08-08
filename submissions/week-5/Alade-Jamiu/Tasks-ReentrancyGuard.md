
**GitHub Repo:** https://github.com/AJTECH001/reentrancy-guard-stylus

**Summary:**  
Implemented a `ReentrancyGuard` module in Rust for Stylus, inspired by OpenZeppelin's `ReentrancyGuard.sol`. The module uses a `StorageBool` to track function entry and prevents reentrant calls with a `non_reentrant` method. An example `Vault` contract demonstrates the guard's effectiveness by comparing a vulnerable `withdraw_unsafe` function with a protected `withdraw` function. The contract includes balance tracking and error handling. Testing instructions are provided to deploy on the Stylus testnet and simulate reentrancy attacks, showcasing the guard's protection.