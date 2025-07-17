## Summary of Chapter 4 (Rust Book)

- Ownership system in Rust, makes memory management possible and efficient. 
- The Stack and the heap are parts of the memory.
- The stack stores data in the order it gets them - LIFO. And the data it stores must be known with a fixed size. 
- The heap is less organized and stores dynamic data with unknown size at compile time. Allocating on the heap returns a pointer and this pointer with a known fixed size is then stored on the stack. 
- Stack operation is faster than allocating on the heap both in storing data and in retrieving data. 
- So understanding ownership aids memory management (stack and heap).
  
  Rules of Ownership
1. In Rust, each value has an owner.
2. There can only be one owner at a time.
3. When the owner goes out-of-scope, the value is dropped.

The major importance of ownership is on the management of the heap part of memory.
This is so because data stored on the stack like integers has a known fixed size and Rust allow copying eg.
```rust
 let x = 10;
 let y = x;
```
x and y are still valid. 

But with data stored on the heap with dynamic size, values are moved (shallow copy), the previous owner & value dropped, and memory freed memory. Ownership is moved to avoid the `double free error`.  

Moving ownership unnecessarily can be tedious and that brings in the concept of `reference and borrowing`.
A reference is an address that leads us to a data that's stored. It remains valid until the lifetime of what is referenced expires or goes out-of-scope. References do not take ownership.