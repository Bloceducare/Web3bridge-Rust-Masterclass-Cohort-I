# 🧪 Stylus ERC-6909 Multi-Token Contract

## 📖 Overview

This project implements the **ERC-6909 multi-token standard** in [Stylus](https://docs.arbitrum.io/stylus), using **Rust**. The contract supports fungible and non-fungible tokens under a single contract instance, providing flexible token management for diverse use cases.

ERC-6909 is designed to unify token interactions, supporting **per-token balances, allowances, and operator approvals**, making it more powerful than simple ERC-20 or ERC-721 implementations.

---

## 📂 Project Structure

```
erc6909_token/
├── src/
│   ├── lib.rs         # Main contract entrypoint
│   └── erc6909.rs     # ERC-6909 implementation logic
├── tests/
│   └── erc6909.rs     # Unit & integration tests
├── README.md          # Project documentation
└── Cargo.toml         # Rust project config
```

---

## ⚙️ Features

* ✅ Per-token **fungible & non-fungible** asset support
* ✅ `total_supply(token_id)` for each token type
* ✅ `balance_of(owner, token_id)` to query ownership
* ✅ `approve(spender, token_id, amount)` for per-token allowances
* ✅ `transfer_from(from, to, token_id, amount)` with allowance checks
* ✅ `set_operator(operator, approved)` for global operator approvals
* ✅ `operator_approval(owner, operator)` to query operator rights
* ✅ Event logs: `TransferSingle`, `ApprovalSingle`
* ✅ Error handling for insufficient balances/allowances
* ⬜ Optional: metadata/URI extension, batch transfers

---

## 🚀 Getting Started

### 1. Install dependencies

Ensure you have Rust and Stylus installed.

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-stylus
```

### 2. Clone repo

```bash
git clone https://github.com/YOUR_USERNAME/erc6909-stylus.git
cd erc6909-stylus
```

### 3. Build contract

```bash
cargo stylus build
```

This compiles the contract into **WASM** for Stylus deployment.

### 4. Run tests

```bash
cargo test
```

Runs unit and integration tests covering transfer flows, approvals, operator rights, and error cases.

---

## 📜 Example Usage

* **Query supply**:

  ```rust
  erc6909.total_supply(token_id);
  ```

* **Check balance**:

  ```rust
  erc6909.balance_of(alice, token_id);
  ```

* **Approve allowance**:

  ```rust
  erc6909.approve(bob, token_id, U256::from(50));
  ```

* **Transfer using allowance**:

  ```rust
  erc6909.transfer_from(alice, carol, token_id, U256::from(30));
  ```

* **Set operator**:

  ```rust
  erc6909.set_operator(operator, true);
  ```

---

## 🧪 Tests Included

* ✅ Transfer updates balances and emits `TransferSingle`
* ✅ Approvals set allowances correctly and emit `ApprovalSingle`
* ✅ `transfer_from` consumes allowances properly
* ✅ Operator approvals allow full token management
* ✅ Reverts on insufficient balance or allowance

---

## 📚 References

* [ERC-6909 Standard Proposal](https://eips.ethereum.org/)
* [ERC-20 Standard](https://eips.ethereum.org/EIPS/eip-20)
* [ERC-721 Standard](https://eips.ethereum.org/EIPS/eip-721)
* [Stylus Documentation](https://docs.arbitrum.io/stylus)

---

## ✅ Submission Format

**GitHub Repo:** [https://github.com/YOUR\_USERNAME/erc6909-stylus](https://github.com/YOUR_USERNAME/erc6909-stylus)

**Summary:**
Implemented ERC-6909 multi-token standard in Rust using Stylus SDK. Supports per-token balances, allowances, operator approvals, and safe transfers.

**Testing:**

* Unit tests for transfer, approve, and operator flows
* Compiled and deployed locally using Stylus CLI
* Verified on-chain behavior with sample calls

**Notes:**

* Optional metadata extension not implemented
* Can extend with URI support or batch operations if needed
