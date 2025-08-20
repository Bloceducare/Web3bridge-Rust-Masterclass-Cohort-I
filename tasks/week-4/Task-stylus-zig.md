
# 📘 Assignment: Zig Stylus Smart Contract (Forked)

## 🎯 Objective

Fork the original [zig-stylus](https://github.com/Stylish-Stylus/zig-stylus) repository and make it your own by:

1. Describing what each file in the `src/` directory does.
2. Building the `main.wasm` contract using Zig.
3. Testing or deploying the contract using the Stylus CLI.

---

## 🗂️ File Descriptions (`src/` folder)

| File | Purpose |
|------|---------|
| `main.zig` | The main contract file. It defines the `user_entrypoint` function that Stylus calls when executing the smart contract. This is where your core logic (e.g., reading inputs, writing outputs, calling functions) lives. |
| `WasmAllocator.zig` | Provides a custom memory allocator that interacts with the Stylus runtime using `memory_grow` for efficient WASM-compatible memory handling. |
| *(Add any other files you include)* | *(Brief explanation of what each one does)* |

---

## 🛠️ Build Instructions

### 1. Install Zig

Make sure you have Zig (e.g., `0.11.0` or newer):

```bash
zig version
````

### 2. Build the Contract

Compile the Zig code to a WASM file compatible with Stylus:

```bash
cd src
zig build-lib main.zig -target wasm32-freestanding -dynamic \
  --export=user_entrypoint --export=mark_unused \
  -OReleaseSmall -o ../main.wasm
```

---

## ✅ Stylus Compatibility Check

Before deploying, verify the `main.wasm` file is compatible with Stylus:

```bash
cargo stylus check --wasm-file=main.wasm
```

---

## 🚀 Deploy (Optional)

If you want to go further, deploy the WASM to the Stylus testnet:

```bash
cargo stylus deploy --private-key=<YOUR_PRIVATE_KEY> --wasm-file=main.wasm
```

---

## 📌 Deliverables

To complete this assignment:

* [ ] Fork the repo
* [ ] Add the file descriptions in `README.md`
* [ ] Build the `main.wasm` file
* [ ] Pass the Stylus compatibility check
* [ ] (Optional) Deploy and test the contract on Stylus

---

## 📎 Submission Format

```md
### Task: zig-stylus-assignment

**GitHub:** https://github.com/YOUR_USERNAME/zig-stylus-fork

**Summary:**  
Forked the zig-stylus repo, documented the files, and compiled the contract to `main.wasm` using Zig. Optionally tested/deployed using the Stylus CLI.
```