### Assignment: `task-node-full-chain-simulation`

### 🎯 Objective

Set up and run a **complete local full-chain simulation** using Arbitrum’s **Nitro Testnode**. This environment includes:

* A dev-mode Ethereum L1 Geth chain
* An L2 Nitro rollup chain (Stylus-enabled by default)
* (Optional) L3 chain support
* Token bridges and a Blockscout explorer

This setup enables you to **locally deploy, test, and debug** Stylus or standard smart contracts in a controlled multi-layer environment ([Arbitrum Docs][1], [Arbitrum Docs][2]).

---

## 🛠️ Setup Instructions

### ✅ Step 1: Install Prerequisites

Ensure the following are installed:

* Docker
* Docker Compose
  (Install via their official websites) ([docs.gelato.network][3])

---

### ✅ Step 2: Clone the Nitro Testnode Repository

```bash
git clone -b release --recurse-submodules https://github.com/OffchainLabs/nitro-testnode.git
cd nitro-testnode
```

This uses the `release` branch which includes the full multi-layer stack([Arbitrum Docs][1]).

---

### ✅ Step 3: Launch the Local Chain

```bash
./test-node.bash --init
```

This initializes:

* A Geth L1 node
* A Stylus-enabled Nitro L2 chain
* (Optionally) L3 dependencies and token bridges
* Deploys all necessary core contracts and generates chain configs ([Arbitrum Docs][1])

---

### 🔁 Step 4: Restart or Reset the Chain

* Restart (inclusive of prior state):

  ```bash
  ./test-node.bash
  ```
* Reset and reinitialize (clears all data):

  ````bash
  ./test-node.bash --init
  ``` :contentReference[oaicite:21]{index=21}
  ````

---

### ⚙️ Optional Features & Queries

| Feature                     | Command                                                                                                            |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Rollup configuration        | `docker exec nitro-testnode-sequencer-1 cat /config/l2_chain_info.json`                                            |
| List config files           | `docker exec nitro-testnode-sequencer-1 ls /config`                                                                |
| Deploy L1–L2 token bridge   | `./test-node.bash --tokenbridge`                                                                                   |
| Deploy L3 chain             | `./test-node.bash --l3node`                                                                                        |
| L3 fee token & L2–L3 bridge | `--l3-fee-token`, `--l3-token-bridge` flags                                                                        |
| Enable Blockscout Explorer  | `./test-node.bash --blockscout` (available at [http://localhost:4000](http://localhost:4000)) ([Arbitrum Docs][1]) |

---

### 🌐 RPC Endpoints & Chain IDs

| Chain           | Chain ID | RPC Endpoint                                    |                 |
| --------------- | -------- | ----------------------------------------------- | --------------- |
| L1 (Geth)       | 1337     | `http://localhost:8545`                         |                 |
| L2 (Nitro)      | 412346   | `http://localhost:8547` / `ws://localhost:8548` |                 |
| L3 (if enabled) | 333333   | `http://localhost:3347`                         | ([NowNodes][4]) |

---

### 👤 Developer Accounts

All provided accounts are publicly known and pre‑funded—for local use only:

* **Dev account**: `0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E` (key `0xb6…9520659`)
* **Sequencer, Validator, L2/L3 rollup owners** are also preset; refer to setup scripts or JSON config for full list ([Arbitrum Docs][2], [GitHub][5])

---

### 💸 Funding via Helper Scripts

Send funds to L2/L1 addresses using:

```bash
./test-node.bash script send-l2 --to <address> --ethamount <amount>
./test-node.bash script send-l1 --to <address> --ethamount <amount>
```

See available scripts with:

````bash
./test-node.bash script --help
``` :contentReference[oaicite:45]{index=45}

---

## 📎 Submission Format

```md
### Task: node-full-chain-simulation

**Objective:** Launch a full local Stylus-enabled chain simulation.

**Completed Steps:**
- Cloned and ran `nitro-testnode --init`
- Interacted with L2 via `localhost:8547`
- Funded addresses using helper scripts
- (Optional) Deployed a contract using portable WASM/A.S./Zig logic

**Proof:**
- Screenshot or terminal log of running nodes
- `l2_chain_info.json` or contract interactions
- [GitHub repo link] if modifications were made
````