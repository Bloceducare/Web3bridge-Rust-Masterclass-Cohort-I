# Task: node-full-chain-simulation

**Objective:** Launch a full local Stylus-enabled chain simulation.

**Completed Steps:**

- ✅ Successfully cloned nitro-testnode repository with `--recurse-submodules`
- ✅ Initialized and launched full chain with `./test-node.bash --init`
- ✅ Verified L1 Geth chain running (Chain ID: 1337, Port: 8545)
- ✅ Verified L2 Nitro chain running (Chain ID: 412346, Port: 8547)
- ✅ Retrieved and analyzed L2 chain configuration
- ✅ Confirmed Stylus support (ArbOS Version 40)
- ✅ Identified all core contract deployments

**Proof of Success:**

**1. L2 Chain Configuration Retrieved:**

```json
{
  "chain-name": "arb-dev-test",
  "parent-chain-id": 1337,
  "parent-chain-is-arbitrum": false,
  "chain-config": {
    "chainId": 412346,
    "arbitrum": {
      "EnableArbOS": true,
      "AllowDebugPrecompiles": true,
      "InitialArbOSVersion": 40,
      "InitialChainOwner": "0x5E1497dD1f08C87b2d8FE23e9AAB6c1De833D927"
    }
  },
  "rollup": {
    "bridge": "0xD31C7fE95D676399240FcB0493beD653e377F7Bf",
    "inbox": "0xA0f3A1a4E2B2Bcb7b48C8527C28098f207572EC1",
    "rollup": "0xE3C3041dbd93a724862137baA32E8D0d311F557C"
  }
}
```

**2. Active Chain Endpoints:**

- **L1 (Geth)**: `http://localhost:8545` (Chain ID: 1337)
- **L2 (Nitro)**: `http://localhost:8547` (Chain ID: 412346)
- **L2 WebSocket**: `ws://localhost:8548`

**3. Key Features Confirmed:**

- ✅ Stylus WASM support enabled (ArbOS v40)
- ✅ Debug precompiles available for testing
- ✅ Complete bridge infrastructure deployed
- ✅ Multi-layer architecture operational

**4. Successful L2 Funding Transaction:**

```javascript
{
  type: 2,
  chainId: 412346,
  hash: '0x625b70327a37125c751299814ba76b002b684657fc5314527243b1b4b135d53a',
  from: '0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E',
  to: '0xEEA8EC07A3642769168D40e3Abd05Af5F1c56c44',
  value: '1.0 ETH',
  gasLimit: 21000,
  maxFeePerGas: '1.7 Gwei'
}
```

**5. Development Ready:**

- ✅ Core contracts deployed and accessible
- ✅ Bridge infrastructure functional  
- ✅ Debug capabilities enabled
- ✅ Funding system operational
- ✅ Ready for smart contract deployment and testing

**Next Steps Available:**

- Deploy custom Stylus contracts (Rust/C++/AssemblyScript)
- Test cross-chain messaging via bridges
- Enable Blockscout explorer with `--blockscout` flag
- Deploy L3 chain with `--l3node` flag

**Status: ✅ COMPLETE**
Full-chain simulation successfully running and ready for development.
  <img width="1901" height="986" alt="Image" src="https://github.com/user-attachments/assets/71a06000-5ea8-4af3-ad14-830cd6fa3004" />

  <img width="902" height="982" alt="Image" src="https://github.com/user-attachments/assets/8b574699-0171-4d90-98ab-5cd309aa08da" />

  <img width="1920" height="988" alt="Image" src="https://github.com/user-attachments/assets/9af9ada4-10eb-415c-b569-4ef152ea981d" />

  <img width="1041" height="667" alt="Image" src="https://github.com/user-attachments/assets/414294f2-12bb-4d1e-a0e8-437365bddf7c" />

  <img width="1920" height="882" alt="Image" src="https://github.com/user-attachments/assets/1aecedbe-020d-46be-bc53-6e41b8c4dcd9" />

## Task-stylus-js

**GitHub:** <https://github.com/emmanuelist/stylus-as-example>

**Description:**
Forked the Stylus AssemblyScript example (Sieve of Eratosthenes) and modified it to return the minimum even prime number (which is always 2), instead of calculating all prime numbers up to a limit.

This simplified example demonstrates how to write and deploy a minimal smart contract with Stylus using AssemblyScript.

<img width="747" height="865" alt="Image" src="https://github.com/user-attachments/assets/0d6ba728-7144-45c4-b197-b9b8d9b8d177" />
