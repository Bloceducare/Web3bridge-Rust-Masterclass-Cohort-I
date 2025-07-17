### Control Flow: 

- If expressions: Used for conditional branching, with conditions that must evaluate to a boolean. Rust does not implicitly convert non-boolean types to booleans.
- Loops: Rust has loop (infinite looping until break), while (condition-based looping), and for (iterating over collections). Loops can return values using break with an expression.
- Loop Labels: You can label loops (e.g., 'outer:) to control nested loop breaks or continues.




### Ownership governs memory management in Rust. Each value has a single owner, and when the owner goes out of scope, the value is dropped, freeing memory automatically.

### Stack: Stores fixed-size data (e.g., integers) in a last-in, first-out structure. Fast due to predictable access.

### Heap: Stores dynamic or variable-sized data (e.g., String). Slower due to allocation and pointer dereferencing.

### Ownership Rules

- Each value has one owner.
- Only one owner exists at a time.
- When the owner goes out of scope, the value is dropped via the drop function.

#### String vs. String Literals:

- String literals (&str) are immutable, hardcoded in the binary, and stored on the stack.
- String is mutable, heap-allocated, and supports dynamic text. Created with String::from.

### Memory Management:

- Rust automatically deallocates heap memory when a variable goes out of scope, avoiding manual allocation/freeing or garbage.

### Copy Trait:

- Types with known, fixed sizes (e.g., i32, bool, char, tuples of Copy types) implement the Copy trait, allowing automatic copying instead of moving.
- Types requiring cleanup (e.g., String) cannot implement Copy.

### Functions and Ownership:

- Passing a value to a function moves ownership (for non-Copy types) or copies it (for Copy types).
- Returning values transfers ownership back. Example: fn takes_ownership(s: String) moves s, invalidating it in the caller.
- To avoid moving, use references

### Borrowing:

- Borrowing refers to using a reference. The referenced value isn’t dropped when the reference goes out of scope.
- Immutable references (&T) allow reading but not modifying.

### Questions

- Memory and Allocation
- Mutable References
- Dangling References
- String Literals as Slices
- Moves
- Clone
- References
