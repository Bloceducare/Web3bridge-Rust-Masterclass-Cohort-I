## 🪙 Task: Implement ERC-20 Token in Stylus (Rust)

### 🎯 Objective

Implement a basic ERC-20 token standard in Stylus using Rust. Your goal is to understand token standards, how smart contracts manage balances, and how to implement EVM-compatible token logic on Stylus.

This task mimics the [OpenZeppelin ERC-20 standard](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol), but your implementation will be written in **Rust** for Stylus.

---

## 📌 Deliverables

* A complete Stylus (Rust) ERC-20 contract with:

  * `name()`, `symbol()`, `decimals()`, `totalSupply()`
  * `balanceOf(address)`
  * `transfer(address, amount)`
  * `approve(address, amount)`
  * `allowance(owner, spender)`
  * `transferFrom(owner, spender, amount)`
* Event logs:

  * `Transfer(from, to, value)`
  * `Approval(owner, spender, value)`
* Unit tests for:

  * Transfers, allowances, reverts (e.g. insufficient balance)
* Optionally: include minting logic in the constructor or a `mint` function (owner-only)

---

## 🛠️ Task Steps

1. **Setup** a Stylus Rust project.

   ```bash
   cargo stylus new erc20_token
   cd erc20_token
   ```

2. **Define the ERC-20 contract**:

   * Use Stylus macros like `#[contract]`, `#[storage]`, `#[external]`
   * Define your state variables:

     ```rust
     #[storage]
     pub struct ERC20 {
         name: String,
         symbol: String,
         decimals: u8,
         total_supply: U256,
         balances: Mapping<Address, U256>,
         allowances: Mapping<(Address, Address), U256>,
     }
     ```

3. **Implement functions**:

   * `fn balance_of(&self, owner: Address) -> U256`
   * `fn transfer(&mut self, to: Address, amount: U256) -> Result<(), String>`
   * `fn approve(&mut self, spender: Address, amount: U256)`
   * `fn transfer_from(&mut self, from: Address, to: Address, amount: U256)`

4. **Emit events** using Stylus logging macros:

   ```rust
   #[event]
   pub struct Transfer {
       from: Address,
       to: Address,
       value: U256,
   }
   ```

5. **Write unit tests** using Stylus-style testing or Rust’s built-in testing for WASM.

---

## 📂 Suggested Folder Structure

```
erc20_token/
├── src/
│   ├── lib.rs         ← Main contract
│   └── erc20.rs       ← Implementation logic
├── tests/
│   └── erc20.rs       ← Unit & integration tests
├── README.md
└── Cargo.toml
```

---

## ✅ Submission Format

````markdown
### Task: stylus-erc20

**GitHub Repo:** https://github.com/YOUR_USERNAME/stylus-erc20

**Summary:**  
Implemented the ERC-20 token standard in Rust for Stylus. Includes all required functions and tests.

**Features:**  
- `name`, `symbol`, `decimals`, `totalSupply`
- `balanceOf`, `transfer`, `approve`, `allowance`, `transferFrom`
- Events: `Transfer`, `Approval`
- Tests for all major functions and failure cases

**Commands:**
```bash
cargo stylus build
cargo test
````

```
d
