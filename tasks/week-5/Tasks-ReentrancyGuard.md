

## 🛡️ Task: Implement ReentrancyGuard with Stylus (Rust)

### 🎯 Objective

Implement a **reentrancy protection utility** in Stylus, inspired by [OpenZeppelin's `ReentrancyGuard.sol`](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/security/ReentrancyGuard.sol). This utility should prevent reentrant calls to protected functions in Stylus-based smart contracts.

---

### 📂 Deliverables

1. A reusable Rust module (e.g. `reentrancy.rs`) that:

   * Defines a `ReentrancyGuard` struct
   * Implements `non_reentrant()` guard logic
   * Maintains and updates internal status to track entry state

2. An example Stylus contract that:

   * Uses `ReentrancyGuard`
   * Demonstrates safe and unsafe behavior with/without the guard
   * Includes simple funds tracking to show reentrant risk

Use OZ pausable as example

---


### 📎 Submission Format

```markdown
### Task: reentrancy-guard-stylus

**GitHub Repo:** https://github.com/YOUR_USERNAME/reentrancy-guard-stylus

**Summary:**  