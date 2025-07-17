# Rust Ownership

## Memory Model
- **Stack**: This is fast, fixed-size data (LIFO)
- **Heap**: This is slower, variable-size data (requires pointers)

## Ownership Rules
1. Each value has exactly one owner
2. Only one owner at a time
3. Value dropped when owner goes out of scope

## Copy vs Clone
- **Copy trait**: Stack data copied automatically (integers, booleans, chars)
- **Clone method**: Explicit deep copy of heap data

## Functions & Ownership
- Passing to function = move or copy (same as assignment)
- Returning from function transfers ownership
- Use references to avoid transferring ownership

---

# References & Borrowing

## Basic References
- Use `&` to create references (borrow without taking ownership)
- References are immutable by default

## Mutable References
- Use `&mut` for mutable references
- Both variable and reference must be declared mutable

## Borrowing Rules
1. **Either** one mutable reference **OR** any number of immutable references
2. References must always be valid (no dangling references)
3. Cannot mix mutable and immutable references simultaneously

## Dangling References
- Compiler prevents returning references to local variables
- Reference scope must not outlive the data it points to

---

# Slices

## String Slices
- Reference to contiguous sequence of elements in a collection
- Type: `&str` (string slice)
- Syntax: `&s[start..end]` or `&s[start..]` or `&s[..end]` or `&s[..]`

## Benefits
- Tied to underlying data (prevents index invalidation bugs)
- Compiler enforces borrowing rules
- More flexible function parameters (`&str` vs `&String`)

## String Literals
- Already string slices (`&str`)
- Immutable references to binary data

## Other Slices
- Work with any collection type
- Array slices: `&[T]`


## Questions
1. When should I use slices vs. iterators vs. other collection methods?
2. How does ownership affect performance in Rust?
3. What are the trade-offs between using references and ownership?