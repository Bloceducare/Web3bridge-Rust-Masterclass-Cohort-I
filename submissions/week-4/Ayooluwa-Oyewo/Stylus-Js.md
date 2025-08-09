# TAsk-stylus-js

**GitHub:** [https://github.com/oyewoas/stylus-as-example](https://github.com/oyewoas/stylus-as-example)

## Description

Forked the Stylus AssemblyScript example (Sieve of Eratosthenes) and modified it to return the **minimum even prime number** (which is always `2`), instead of calculating all prime numbers up to a limit.

This simplified example demonstrates how to write and deploy a minimal smart contract with **Stylus** using **AssemblyScript**.
# Deployment

Deploy the contract using:

```bash
cargo stylus deploy --wasm-file ./build/release.wasm --private-key=YOUR_PRIVATE_KEY --no-verify
```

Deployment output:

```
stripped custom section from user wasm to remove any sensitive data
contract size: 845 B (845 bytes)
wasm data fee: 0.000055 ETH (originally 0.000046 ETH with 20% bump)
deployed code at address: 0xe1080224b632a93951a7cfa33eeea9fd81558b5e
deployment tx hash: 0xcd51c00707d288bca0c477468272c6605486a9be43530b45ac176fd6e3ce7044
contract activated and ready onchain with tx hash: 0x695e9935446ce80714aae1eb44fa1e2e84aa2d15b6b0e4c4a48fe4789ba8e6b3
```

> 📌 **Note**: It is recommended to cache your contract with:
>
> ```bash
> cargo stylus cache bid e1080224b632a93951a7cfa33eeea9fd81558b5e 0
> ```
>
> Cached contracts benefit from cheaper calls. [Read more about caching contracts →](https://docs.arbitrum.io/stylus/how-tos/caching-contracts)

## Onchain Testing

Run the test with:

```bash
yarn test:onchain 56
```

Test output:

```
*************************
* Stylus onchain tester *
*************************

------------------
| Start contract |
------------------

-----------------
Input: 56
Calling contract...
Result: 2
-----------------
```

✅ The result `2` confirms that the contract correctly returns the minimum even prime number.