# Summary of what I understand so far, Day 2 - Rust MasterClass

## Chapter 3 - Common Programming Concepts

### Variables and Mutability

- By default variables are **immutable**. Use `mut` to allow reassignment.
- **Constants** are always immutable, require a type annotation, and live for the **entire program scope**.
- **Shadowing**: You can redeclare a variable with `let`, reusing the name. Each `let x = …` creates a new binding, allowing type or value changes without using mut.

### Data Types

- **Static Typing**: Rust infers types but you must annotate in ambiguous cases. All values have a type known at compile time
- **Scalar Types**: integers, floating-point, bool, and char (4-byte Unicode). Use underscores for readability (e.g. 1_000). Integer overflow panics in debug mode or wraps on release by default.
- **Compound Types**:
    - **Tuples**: fixed-size heterogeneous collections, e.g. `let t: (i32,f64,char) = (3, 2.5, 'a');`. Access via destructuring or dot-index (e.g. `let (x,y,z)= t ;` `let y = t.1;`).
    - **Arrays**: fixed-size homogeneous collections, e.g. `let a: [i32; 3] = [1,2,3];`. Access by index (e.g. `a[0])` and use `[val; len]` to repeat a value.

### Functions

- **Statements vs Expressions**: Variable assignments and function definitions are statements (no return value). Expressions evaluate to values and can be used in statements. A trailing expression (without ;) in a block is returned.

### Control Flow

- **if/else**: Conditions must be `bool` (no implicit truthiness)
- **if as Expression**: `if` returns a value. All branches must yield the same type, otherwise compile error is thrown
- **Repetition with Loops**: Rust has `loop`, `while`, and `for`.

## Chapter 4 - Understanding Ownership

### Ownership Rules

- While Rust is not a full systems programming language, it takes memory management strictly and seriously. Ownership is its way of enforcing memory management unlike grabage collection in traditional systems languages. Both the stack and the heap are parts of memory available to our code to use at runtime
- **Stack**: It is fast (due to allocator access time), structured, stores known, fixed size data and uses LIFO (last in, first out).
    - Primitives, e.g. `i32`, `u64`, `bool`, `char`, `arr` [ (of primitives) ] etc
    - Copy types (small, simple data that implements the Copy trait).
    - Function call frames, including: Arguments, Return addresses, Local variables with known size
- **Heap**: Stores data with an unknown size at compile time/ changing size.
    - `String`
    - `Vec<T>`
    - Boxed types (`Box<T>`)
    - Complex types that contain heap data store their pointer (address) on the stack, but the actual data is on the heap.
- **Rules**: Each value has one owner (a variable) at a time. When the owner goes out of scope, the value is dropped and memory is freed.
- **Move Semantics**: Assigning or passing non-Copy types (like String) transfers ownership (a move). 

Example:
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved into s2; s1 is now invalid
```
After this, using s1 is a compile-time error (it no longer owns data)
- **Copy Types**: Simple scalar types (e.g. integers, booleans, chars, tuples of them) implement the `Copy` trait. Assigning these duplicates the value on the stack, so both variables remain usable.
- Use `.clone()` to explicitly deep-copy heap data when needed (at performance cost). Rust never does implicit deep copies, so moves are cheap

### References and Borrowing

- `&T` creates an immutable reference (borrow) to `T`; it does not transfer ownership. Multiple immutable references are allowed.
- `&mut T` creates a mutable borrow, allowing in-place mutation. The original value must be `mut`. For example:
```rust
fn change(s: &mut String) { s.push_str(" world"); }
let mut s = String::from("hello");
change(&mut s); // OK
```
- **Borrow Rules**: At any point you can have either one mutable reference or any number of immutable references to a value, but not both. Attempting to have two simultaneous mutable references, or a mutable reference while immutable ones exist, is a compile-time error

- **No Dangling References**: Rust ensures references never outlive the data they point to. The compiler forbids returning references to local stack data (no “dangling pointer”)

### Slice Types

- A slice is a reference to a contiguous sequence within a collection, without taking ownership.

- String slices (&str): e.g. `&s[0..i]` refers to part of a String or string literal. Example function returning the first word:
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[0..i]; }
    }
    &s[..]
}
```
This works for String slices or string literals (which are `&str`)
- Array slices (`&[T]`): similarly, `&array[a..b]` yields a slice of elements. E.g.
```rust
let a = [1,2,3,4,5];
let slice = &a[1..3]; // &[2, 3]
```
The slice holds a pointer and a length, without copying the data