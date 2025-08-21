## File Descriptions (`src/` folder)

| File | Purpose |
|------|---------|
| `main.zig` | The main smart contract file. Defines the `user_entrypoint` function, which the Stylus runtime calls to execute the contract. It handles input processing, stores and retrieves data using `Hostio` storage functions, and outputs results. |
| `WasmAllocator.zig` | Implements a custom WebAssembly memory allocator. Uses the Stylus runtime’s `memory_grow` function to manage memory efficiently, supporting small and large allocations with free lists for reuse. |
| `Helpers/Hostio.zig` | Provides utility functions to interact with the Stylus runtime, including reading input arguments (`read_args`), writing output results (`write_result`), and managing persistent storage (`storage_store_bytes32`, `storage_load_bytes32`). |