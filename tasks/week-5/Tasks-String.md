# Assignment: Implement `Strings.sol` Functionality in Stylus

## 🎯 Objective

Recreate the core utility functions from OpenZeppelin’s `Strings.sol` library in **Stylus (Rust + Alloy SDK)**, providing a utility module for string conversions, especially:

* `toString(uint256)` → decimal string
* `toHexString(uint256)` → hexadecimal string
* `toHexString(uint256, length)` → fixed‑length hex string with `0x`

Your Rust-based module should mimic the behavior and interface of the Solidity version as closely as possible.

---

### ⚙️ Step 1: Review OpenZeppelin `Strings.sol`

The Solidity library includes:

* `toString(uint256 value)` – converts value to ASCII decimal string ([forum.openzeppelin.com][1], [immutablesoft.github.io][2], [forum.openzeppelin.com][3], [github.com][4])
* `toHexString(uint256 value)` – converts to variable‐length hex with `0x` prefix
* `toHexString(uint256 value, uint256 length)` – converts with specified hex length ([immutablesoft.github.io][2])

These functions are often used for building token URIs, serialization, or human-readable output.

---

## 🛠️ Task Specifications

### 1. Create a Stylus Rust “Strings” module

* Implement `to_string(value: U256) -> String`
* Implement `to_hex_string(value: U256) -> String`
* Implement `to_hex_string_fixed(value: U256, len: usize) -> String`

### 2. Write unit tests that verify:

* Converting various numeric values (0, 1, small, large) to decimal strings
* Converting to hex strings and fixed-length hex (with leading zeros)
* Edge cases: zero value, maximum 256-bit value

### 3. Package as a utility library

* Make the module importable via:

  ```rust
  use strings_utils::to_string;
  ```
* Document the interface in `README.md` with examples.

### 4. Demo in a Stylus contract

* Create a simple contract that:

  * Takes an input `uint256`
  * Returns its decimal or hex string (via your utility)

---

## 📂 Repository Structure (suggested)

```
strings_utils/
├── src/
│   └── lib.rs         ← `to_string` & `to_hex_string` implementations
├── tests/
│   └── strings.rs     ← unit tests
└── README.md          ← how to use
```

---

## 📎 Submission Format

````markdown
### Task: strings-utils-stylus

**GitHub Repo:** https://github.com/YOUR_USERNAME/strings-utils-stylus

**Summary:**  
Reimplemented OpenZeppelin's `Strings.sol` functions (`toString`, `toHexString`) in Rust using the Stylus SDK.  
Includes unit tests and a demo contract showcasing its use.

**Features:**  
- `to_string(U256)` → decimal  
- `to_hex_string(U256)`  
- `to_hex_string_fixed(U256, usize)`  
- Demo contract that returns strings from input values  
- Thorough tests for typical and edge case values

**Tested on:**  
- Unit tests passed  
- Demo contract deployed on local Stylus node with `to_string(to_hex_string())` output verified

**Usage example in contract:**
```rust
let val: U256 = msg::get_arg(0)?;
let s = strings_utils::to_string(val);
msg::return_string(s);
````

````

---

## 🔧 Example Snippet (Rust)

```rust
pub fn to_string(value: U256) -> String {
    let mut v = value;
    if v.is_zero() {
        return "0".to_string();
    }
    let mut buf = Vec::new();
    while v > U256::zero() {
        let digit = (v % U256::from(10)).as_u64() as u8;
        buf.push(b'0' + digit);
        v /= U256::from(10);
    }
    buf.reverse();
    String::from_utf8(buf).unwrap()
}
d