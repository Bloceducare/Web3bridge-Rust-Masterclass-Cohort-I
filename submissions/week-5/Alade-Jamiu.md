
**GitHub Repo:** https://github.com/AJTECH001/stylus_erc20_token

**Summary:**  
Implemented the ERC-20 token standard in Rust for Stylus. Includes all required functions and tests, with a constructor for minting an initial supply to the deployer.

**Features:**  
- `name`, `symbol`, `decimals`, `totalSupply`
- `balanceOf`, `transfer`, `approve`, `allowance`, `transferFrom`
- Events: `Transfer`, `Approval`
- Tests for initialization, transfers, approvals, and failure cases (insufficient balance, insufficient allowance)





**GitHub Repo:** https://github.com/AJTECH001/ERC-6909_Stylus_Contract

**Summary:**  
Implemented ERC-6909 multi-token standard in Rust using Stylus SDK.  
Supports per-token balances, allowances, operator approvals, and safe transfers.  
Includes event emission and error handling.

**Testing:**  
- Unit tests for transfer, approve, and operator flows  
- Compiled and deployed locally using Stylus CLI  
- Verified on-chain behavior with sample calls



**GitHub Repo:** https://github.com/AJTECH001/reentrancy-guard-stylus

**Summary:**  
Implemented a `ReentrancyGuard` module in Rust for Stylus, inspired by OpenZeppelin's `ReentrancyGuard.sol`. The module uses a `StorageBool` to track function entry and prevents reentrant calls with a `non_reentrant` method. An example `Vault` contract demonstrates the guard's effectiveness by comparing a vulnerable `withdraw_unsafe` function with a protected `withdraw` function. The contract includes balance tracking and error handling. Testing instructions are provided to deploy on the Stylus testnet and simulate reentrancy attacks, showcasing the guard's protection.



**GitHub Repo:** https://github.com/AJTECH001/stylus_strings_utils


**Summary:**  
This project reimplements OpenZeppelin's `Strings.sol` library in Rust for the Stylus SDK, providing utility functions to convert `U256` values into decimal and hexadecimal strings. It includes `to_string` for decimal conversion, `to_hex_string` for variable-length hex strings, and `to_hex_string_fixed` for fixed-length hex strings with a `0x` prefix. The library is thoroughly tested with unit tests covering typical and edge cases, and a demo Stylus contract demonstrates its usage in a smart contract environment.