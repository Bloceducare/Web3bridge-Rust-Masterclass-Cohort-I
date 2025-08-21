/* eslint-disable prettier/prettier */
// @ts-nocheck Supresses "Decorators are not allowed here" error
// TODO: Fix "Decorators are not allowed here" warning
// Once fixed, remove the .prettierignore file too

// Define the hooks as external functions that the AssemblyScript module can call.
// Based on https://github.com/OffchainLabs/stylus-sdk-rs/blob/main/stylus-sdk/src/hostio.rs

/**
 * Gets the ETH balance in wei of the account at the given address.
 * The semantics are equivalent to that of the EVM’s [`BALANCE`] opcode.
 * 
 * [`BALANCE`]: https://www.evm.codes/#31
 */
@external("vm_hooks", "account_balance")
export declare function account_balance(address: u32, dest: u32): void;

/**
 * Gets a subset of the code from the account at the given address. The semantics are identical to that
 * of the EVM's [`EXT_CODE_COPY`] opcode, aside from one small detail: the write to the buffer `dest` will
 * stop after the last byte is written. This is unlike the EVM, which right pads with zeros in this scenario.
 * The return value is the number of bytes written, which allows the caller to detect if this has occurred.
 * 
 * [`EXT_CODE_COPY`]: https://www.evm.codes/#3C
 */
@external("vm_hooks", "account_code")
export declare function account_code(address: u32, offset: u32, size: u32, dest: u32): u32;

/**
 * Gets the size of the code in bytes at the given address. The semantics are equivalent
 * to that of the EVM's [`EXT_CODESIZE`].
 * 
 * [`EXT_CODESIZE`]: https://www.evm.codes/#3B
 */
@external("vm_hooks", "account_code_size")
export declare function account_code_size(address: u32): u32;

/**
 * Gets the code hash of the account at the given address. The semantics are equivalent
 * to that of the EVM's [`EXT_CODEHASH`] opcode. Note that the code hash of an account without
 * code will be the empty hash
 * `keccak("") = c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470`.
 * 
 * [`EXT_CODEHASH`]: https://www.evm.codes/#3F
 */
@external("vm_hooks", "account_codehash")
export declare function account_codehash(address: u32, dest: u32): void;

/**
 * Gets the basefee of the current block. The semantics are equivalent to that of the EVM's
 * [`BASEFEE`] opcode.
 * 
 * [`BASEFEE`]: https://www.evm.codes/#48
 */
@external("vm_hooks", "block_basefee")
export declare function block_basefee(basefee: u32): void;

/**
 * Gets the coinbase of the current block, which on Arbitrum chains is the L1 batch poster's
 * address. This differs from Ethereum where the validator including the transaction
 * determines the coinbase.
 */
@external("vm_hooks", "block_coinbase")
export declare function block_coinbase(coinbase: u32): void;

/**
 * Gets the gas limit of the current block. The semantics are equivalent to that of the EVM's
 * [`GAS_LIMIT`] opcode. Note that as of the time of this writing, `evm.codes` incorrectly
 * implies that the opcode returns the gas limit of the current transaction.  When in doubt,
 * consult [`The Ethereum Yellow Paper`].
 * 
 * [`GAS_LIMIT`]: https://www.evm.codes/#45
 * [`The Ethereum Yellow Paper`]: https://ethereum.github.io/yellowpaper/paper.pdf
 */
@external("vm_hooks", "block_gas_limit")
export declare function block_gas_limit(): u64;

/**
 * Gets a bounded estimate of the L1 block number at which the Sequencer sequenced the
 * transaction. See [`Block Numbers and Time`] for more information on how this value is
 * determined.
 * 
 * [`Block Numbers and Time`]: https://developer.arbitrum.io/time
 */
@external("vm_hooks", "block_number")
export declare function block_number(): u64;

/**
 * Gets a bounded estimate of the Unix timestamp at which the Sequencer sequenced the
 * transaction. See [`Block Numbers and Time`] for more information on how this value is
 * determined.
 * 
 * [`Block Numbers and Time`]: https://developer.arbitrum.io/time
 */
@external("vm_hooks", "block_timestamp")
export declare function block_timestamp(): u64;

/**
 * Calls the contract at the given address with options for passing value and to limit the
 * amount of gas supplied. The return status indicates whether the call succeeded, and is
 * nonzero on failure.
 * 
 * In both cases `return_data_len` will store the length of the result, the bytes of which can
 * be read via the `read_return_data` hostio. The bytes are not returned directly so that the
 * programmer can potentially save gas by choosing which subset of the return result they'd
 * like to copy.
 * 
 * The semantics are equivalent to that of the EVM's [`CALL`] opcode, including callvalue
 * stipends and the 63/64 gas rule. This means that supplying the `u64::MAX` gas can be used
 * to send as much as possible.
 * 
 * [`CALL`]: https://www.evm.codes/#f1
 */
@external("vm_hooks", "call_contract")
export declare function call_contract(
    contract: u32,
    calldata: u32,
    calldata_len: u32,
    value: u32,
    gas: u64,
    return_data_len: u32
): u8;

/**
 * Gets the unique chain identifier of the Arbitrum chain. The semantics are equivalent to
 * that of the EVM's [`CHAIN_ID`] opcode.
 * 
 * [`CHAIN_ID`]: https://www.evm.codes/#46
 */
@external("vm_hooks", "chainid")
export declare function chainid(): u64;

/**
 * Gets the address of the current program. The semantics are equivalent to that of the EVM's
 * [`ADDRESS`] opcode.
 * 
 * [`ADDRESS`]: https://www.evm.codes/#30
 */
@external("vm_hooks", "contract_address")
export declare function contract_address(address: u32): void;

/**
 * Deploys a new contract using the init code provided, which the EVM executes to construct
 * the code of the newly deployed contract. The init code must be written in EVM bytecode, but
 * the code it deploys can be that of a Stylus program. The code returned will be treated as
 * WASM if it begins with the EOF-inspired header `0xEFF000`. Otherwise the code will be
 * interpreted as that of a traditional EVM-style contract. See [`Deploying Stylus Programs`]
 * for more information on writing init code.
 * 
 * On success, this hostio returns the address of the newly created account whose address is
 * a function of the sender and nonce. On failure the address will be `0`, `return_data_len`
 * will store the length of the revert data, the bytes of which can be read via the
 * `read_return_data` hostio. The semantics are equivalent to that of the EVM's [`CREATE`]
 * opcode, which notably includes the exact address returned.
 * 
 * [`Deploying Stylus Programs`]: https://developer.arbitrum.io/TODO
 * [`CREATE`]: https://www.evm.codes/#f0
 */
@external("vm_hooks", "create1")
export declare function create1(
    code: u32,
    code_len: u32,
    endowment: u32,
    contract: u32,
    revert_data_len: u32
): void;

/**
 * Deploys a new contract using the init code provided, which the EVM executes to construct
 * the code of the newly deployed contract. The init code must be written in EVM bytecode, but
 * the code it deploys can be that of a Stylus program. The code returned will be treated as
 * WASM if it begins with the EOF-inspired header `0xEFF000`. Otherwise the code will be
 * interpreted as that of a traditional EVM-style contract. See [`Deploying Stylus Programs`]
 * for more information on writing init code.
 * 
 * On success, this hostio returns the address of the newly created account whose address is a
 * function of the sender, salt, and init code. On failure the address will be `0`,
 * `return_data_len` will store the length of the revert data, the bytes of which can be read
 * via the `read_return_data` hostio. The semantics are equivalent to that of the EVM's
 * `[CREATE2`] opcode, which notably includes the exact address returned.
 * 
 * [`Deploying Stylus Programs`]: https://developer.arbitrum.io/TODO
 * [`CREATE2`]: https://www.evm.codes/#f5
 */
@external("vm_hooks", "create2")
export declare function create2(
    code: u32,
    code_len: u32,
    endowment: u32,
    salt: u32,
    contract: u32,
    revert_data_len: u32
): void;

/**
 * Delegate calls the contract at the given address, with the option to limit the amount of
 * gas supplied. The return status indicates whether the call succeeded, and is nonzero on
 * failure.
 * 
 * In both cases `return_data_len` will store the length of the result, the bytes of which
 * can be read via the `read_return_data` hostio. The bytes are not returned directly so that
 * the programmer can potentially save gas by choosing which subset of the return result
 * they'd like to copy.
 * 
 * The semantics are equivalent to that of the EVM's [`DELEGATE_CALL`] opcode, including the
 * 63/64 gas rule. This means that supplying `u64::MAX` gas can be used to send as much as
 * possible.
 * 
 * [`DELEGATE_CALL`]: https://www.evm.codes/#F4
 */
@external("vm_hooks", "delegate_call_contract")
export declare function delegate_call_contract(
    contract: u32,
    calldata: u32,
    calldata_len: u32,
    gas: u64,
    return_data_len: u32
): u8;

/**
 * Emits an EVM log with the given number of topics and data, the first bytes of which should
 * be the 32-byte-aligned topic data. The semantics are equivalent to that of the EVM's
 * [`LOG0`], [`LOG1`], [`LOG2`], [`LOG3`], and [`LOG4`] opcodes based on the number of topics
 * specified. Requesting more than `4` topics will induce a revert.
 * 
 * [`LOG0`]: https://www.evm.codes/#a0
 * [`LOG1`]: https://www.evm.codes/#a1
 * [`LOG2`]: https://www.evm.codes/#a2
 * [`LOG3`]: https://www.evm.codes/#a3
 * [`LOG4`]: https://www.evm.codes/#a4
 */
@external("vm_hooks", "emit_log")
export declare function emit_log(data: u32, len: u32, topics: u32): void;

/**
 * Gets the amount of gas left after paying for the cost of this hostio. The semantics are
 * equivalent to that of the EVM's [`GAS`] opcode.
 * 
 *  [`GAS`]: https://www.evm.codes/#5a
 */
@external("vm_hooks", "evm_gas_left")
export declare function evm_gas_left(): u64;

/**
 * Gets the amount of ink remaining after paying for the cost of this hostio. The semantics
 * are equivalent to that of the EVM's [`GAS`] opcode, except the units are in ink. See
 * [`Ink and Gas`] for more information on Stylus's compute pricing.
 * 
 * [`GAS`]: https://www.evm.codes/#5a
 * [`Ink and Gas`]: https://developer.arbitrum.io/TODO
 */
@external("vm_hooks", "evm_ink_left")
export declare function evm_ink_left(): u64;

/**
 * The `ENTRYPOINT` macro handles importing this hostio, which is required if the
 * program's memory grows. Otherwise compilation through the `ArbWasm` precompile will revert.
 * Internally the Stylus VM forces calls to this hostio whenever new WASM pages are allocated.
 * Calls made voluntarily will unproductively consume gas.
 */
@external("vm_hooks", "pay_for_memory_grow")
export declare function pay_for_memory_grow(new_pages: u16): void;

/**
 * Whether the current call is reentrant.
 */
@external("vm_hooks", "msg_reentrant")
export declare function msg_reentrant(): bool;

/** 
 * Gets the address of the account that called the program. For normal L2-to-L2 transactions
 * the semantics are equivalent to that of the EVM's [`CALLER`] opcode, including in cases
 * arising from [`DELEGATE_CALL`].
 * 
 * For L1-to-L2 retryable ticket transactions, the top-level sender's address will be aliased.
 * See [`Retryable Ticket Address Aliasing`] for more information on how this works.
 * 
 * [`CALLER`]: https://www.evm.codes/#33
 * [`DELEGATE_CALL`]: https://www.evm.codes/#f4
 * [`Retryable Ticket Address Aliasing`]: https://developer.arbitrum.io/arbos/l1-to-l2-messaging#address-aliasing
 */
@external("vm_hooks", "msg_sender")
export declare function msg_sender(sender: u32): void;

/**
 * Get the ETH value in wei sent to the program. The semantics are equivalent to that of the
 * EVM's [`CALLVALUE`] opcode.
 * 
 * [`CALLVALUE`]: https://www.evm.codes/#34
 */
@external("vm_hooks", "msg_value")
export declare function msg_value(value: u32): void;

/**
 * Efficiently computes the [`keccak256`] hash of the given preimage.
 * The semantics are equivalent to that of the EVM's [`SHA3`] opcode.
 * 
 * [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
 * [`SHA3`]: https://www.evm.codes/#20
 */
@external("vm_hooks", "native_keccak256")
export declare function native_keccak256(bytes: u32, len: u32, output: u32): void;

/**
 * Reads the program calldata. The semantics are equivalent to that of the EVM's
 * [`CALLDATA_COPY`] opcode when requesting the entirety of the current call's calldata.
 * 
 * [`CALLDATA_COPY`]: https://www.evm.codes/#37
 */
@external("vm_hooks", "read_args")
export declare function read_args(data: u32): void;

/**
 * Copies the bytes of the last EVM call or deployment return result. Reverts if out of
 * bounds. The semantics are equivalent to that of the EVM's [`RETURN_DATA_COPY`] opcode.
 * 
 * [`RETURN_DATA_COPY`]: https://www.evm.codes/#3e
 */
@external("vm_hooks", "read_return_data")
export declare function read_return_data(dest: u32, offset: u32, size: u32): u32;

/**
 * Returns the length of the last EVM call or deployment return result, or `0` if neither have
 * happened during the program's execution. The semantics are equivalent to that of the EVM's
 * [`RETURN_DATA_SIZE`] opcode.
 *
 * [`RETURN_DATA_SIZE`]: https://www.evm.codes/#3d
 */
@external("vm_hooks", "return_data_size")
export declare function return_data_size(): u32;

/**
 * Static calls the contract at the given address, with the option to limit the amount of gas
 * supplied. The return status indicates whether the call succeeded, and is nonzero on
 * failure.
 * 
 * In both cases `return_data_len` will store the length of the result, the bytes of which can
 * be read via the `read_return_data` hostio. The bytes are not returned directly so that the
 * programmer can potentially save gas by choosing which subset of the return result they'd
 * like to copy.
 * 
 * The semantics are equivalent to that of the EVM's [`STATIC_CALL`] opcode, including the
 * 63/64 gas rule. This means that supplying `u64::MAX` gas can be used to send as much as
 * possible.
 * 
 * [`STATIC_CALL`]: https://www.evm.codes/#FA
 */
@external("vm_hooks", "static_call_contract")
export declare function static_call_contract(
    contract: u32,
    calldata: u32,
    calldata_len: u32,
    gas: u64,
    return_data_len: u32
): u8;

/**
 * Reads a 32-byte value from permanent storage. Stylus's storage format is identical to
 * that of the EVM. This means that, under the hood, this hostio is accessing the 32-byte
 * value stored in the EVM state trie at offset `key`, which will be `0` when not previously
 * set. The semantics, then, are equivalent to that of the EVM's [`SLOAD`] opcode.
 * 
 * [`SLOAD`]: https://www.evm.codes/#54
 */
@external("vm_hooks", "storage_load_bytes32")
export declare function storage_load_bytes32(key: u32, dest: u32): void;

/**
 * Stores a 32-byte value to permanent storage. Stylus's storage format is identical to that
 * of the EVM. This means that, under the hood, this hostio is storing a 32-byte value into
 * the EVM state trie at offset `key`. Furthermore, refunds are tabulated exactly as in the
 * EVM. The semantics, then, are equivalent to that of the EVM's [`SSTORE`] opcode.
 * 
 * Note: because the value is cached, one must call `storage_flush_cache` to persist it.
 * 
 * [`SSTORE`]: https://www.evm.codes/#55
 */
@external("vm_hooks", "storage_store_bytes32")
export declare function storage_store_bytes32(key: u32, value: u32): void;

/**
 * Persists any dirty values in the storage cache to the EVM state trie, dropping the cache entirely if requested.
 * Analogous to repeated invocations of [`SSTORE`].
 *
 * [`SSTORE`]: https://www.evm.codes/#55
 */
@external("vm_hooks", "storage_flush_cache")
export declare function storage_flush_cache(clear: bool): void;

/**
 * Gets the gas price in wei per gas, which on Arbitrum chains equals the basefee. The
 *  semantics are equivalent to that of the EVM's [`GAS_PRICE`] opcode.
 * 
 *  [`GAS_PRICE`]: https://www.evm.codes/#3A
 */
@external("vm_hooks", "tx_gas_price")
export declare function tx_gas_price(gas_price: u32): void;

/**
 * Gets the price of ink in evm gas basis points. See [`Ink and Gas`] for more information on
 * Stylus's compute-pricing model.
 * 
 * [`Ink and Gas`]: https://developer.arbitrum.io/TODO
 */
@external("vm_hooks", "tx_ink_price")
export declare function tx_ink_price(): u32;

/**
 * Gets the top-level sender of the transaction. The semantics are equivalent to that of the
 * EVM's [`ORIGIN`] opcode.
 * 
 * [`ORIGIN`]: https://www.evm.codes/#32
 */
@external("vm_hooks", "tx_origin")
export declare function tx_origin(origin: u32): void;

/**
 * Writes the final return data. If not called before the program exists, the return data will
 * be 0 bytes long. Note that this hostio does not cause the program to exit, which happens
 * naturally when [`user_entrypoint`] returns.
 */
@external("vm_hooks", "write_result")
export declare function write_result(data: u32, len: u32): void;
