# Chapter 4 Summary: Understanding Ownership

## Ownership

### What I learned
**Ownership** is a set of rules that govern how Rust manages memory. Unlike languages with garbage collection or manual memory management, Rust uses ownership rules that the compiler checks at compile time.

- **Stack**: Stores values in last-in-first-out order. Fast access, fixed-size data only
- **Heap**: Less organized, stores variable-size data. Allocator finds space and returns a pointer

### Ownership Rules
1. Each value in Rust has an **owner**
2. There can only be **one owner** at a time
3. When the owner goes **out of scope**, the value will be **dropped**

#### Variable Scope
Variables are valid from declaration until the end of their scope:

#### The String Type
- String literals (`"hello"`) are immutable and stored in the binary


#### Memory and Allocation
- **Stack data**: Fixed size, automatically managed (Copy types)
- **Heap data**: Variable size, managed through ownership
- When a variable goes out of scope, Rust calls the `drop` function

#### Move Semantics
- Rust **moves** ownership rather than copying heap data
- Original variable becomes invalid after move

#### Clone vs Copy
- **Clone**: Explicitly deep copy heap data
- **Copy trait**: Stack-only data (integers, booleans, chars) are automatically copied

#### Functions and Ownership
- Passing values to functions moves or copies them (same as assignment)
- Returning values transfers ownership to the caller
- Taking and returning ownership for every function is tedious, so we use references

## References and Borrowing

### What I learned
**References** allow you to refer to values without taking ownership of them. **Borrowing** is the act of creating references.

### Reference Rules
- References are **immutable by default**
- You cannot modify something you're borrowing (unless it's a mutable reference)
- To create a mutable referenc, you use the `&mut` key word


### Borrowing Restrictions
1. **Only one mutable reference** to data in a particular scope
2. **Cannot mix mutable and immutable references** simultaneously
3. **Multiple immutable references** are allowed

These rules prevent **data races** at compile time, which occurs when:
- Two or more pointers access the same data simultaneously
- At least one pointer is writing to the data  
- There's no synchronization mechanism, to synchronise access to the data.

### Reference Scopes
References are valid from introduction until last use

### Dangling References
Rust prevents dangling references (pointers to deallocated memory), by basically preventing you from creating a reference to a variable that WILL go out of scope.

### The Rules of References
1. At any given time, you can have **either**:
   - One mutable reference, **OR**
   - Any number of immutable references
2. References must **always be valid**

## The Slice Type

### What I learned
**Slices** let you reference a contiguous sequence of elements in a collection rather than the whole collection. Slices are references, so they don't have ownership.

### The Problem Slices Solve
Without slices, working with parts of collections is error-prone

### String Slices
A **string slice** (`&str`) is a reference to part of a `String`:
A **string slice** is always immutable, it cannot be modified.
String literals are slices pointing to specific points in the binary.

### Other Slices
Slices work with arrays and other collections too, they are not limited to strings.

### What I do not understand.
Nuthin at the time of writing.

