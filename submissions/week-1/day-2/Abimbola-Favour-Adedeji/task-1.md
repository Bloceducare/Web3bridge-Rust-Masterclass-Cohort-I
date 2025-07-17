# Summary of Chapter 3 (Common Programming Concepts)

  - Variables use let and are *immutable by default*; mut makes them mutable.
  - *Constants* (const) must include a type, are compile-time evaluated, and immutable.
  - *Static variables* exist for the program’s lifetime; mutable statics require unsafe access.
  - *Shadowing* allows redeclaring variables (with same name) even changing types.
  - Rust is statically typed; type inference is common, but annotations may be needed.
  - *Primitive (scalar) types* include:
    - Integers: signed (i8–i128, isize) & unsigned (u8–u128, usize).
    - Floats: f32, f64 (default).
    - Boolean: bool (true/false).
    - Character: char (4 byte Unicode scalar).
  - *Compound types*:
    - *Tuples*: fixed-length, mixed types, supports destructuring and indexing.
    - *Arrays*: fixed-length, same type elements, indexed, panic on out-of-bounds.
  - Function is defined with fn, parameters need types, return type after ->.
  - The *last expression is returned implicitly* (no semicolon).
  - Rust differentiates:
    - *Statements*: do something; no return value.
    - *Expressions*: evaluate to values, used in statements.
  - Line comments: //, block comments: /* … */.
  - Doc comments:
    - /// for documenting items.
    - //! for module-level documentation.
  - if is an expression requiring a bool. No truthy/falsy conversions.
  - Variants:
    - *if let* — conditional assignment from enums like Option.
    - *loop* — infinite loops, can be labeled, and return a value via break val.
    - *while* — conditional loop. This will continue looping while a condition is truthy.
    - *for* — iterator-based looping (e.g. ranges, .iter()).
  - Chapter mentions key reserved keywords (let, mut, fn, const, if, loop, etc.) that are foundational to Rust's syntax.


# Summary of Chapter 4 (Understanding Ownership)
  - A scope is the range within a program for which an item is valid.
  - When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.
  - The length is how much memory, in bytes, the contents of the String are currently using.
  - The capacity is the total amount of memory, in bytes, that the String has received from the allocator
  ```rust
    let s1 = String::from("hello");
    let s2 = s1;
  ```
  - In the example above, the s1 stored on the stack is copied into the s2 variable but they’re both pointing to the same heap location in memory. If s1 go out of scope its memory would be cleared on the heap which would cause two major problems. The first problem is that s2 would be pointing to a cleared(non-existing) location on the heap which has been cleared when s1 goes out of scope. The second problem is that whenever s2 goes out scope, Rust would try to clear its memory on the heap which would result to double free error. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
  - Instead of copying s1 into s2 Rust would move s1 into s2 variable considering s1 not valid anymore, With only s2  being valid, when s2 goes out of scope it alone will free the memory.
  - The concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move.
  - In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
  - If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
  - When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately.
  - A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.
  - Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
  - Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. Any attempt to create two mutable references will fail
  - The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. The benefit of having this restriction is that Rust can prevent data races at compile time
  - A data race is similar to a race condition and happens when these three behaviors occur:
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data.
    - There’s no mechanism being used to synchronize access to the data.
  - Dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.
