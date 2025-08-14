
# Chapter 3 & 4 Study Summary

## Chapter 3: Common Programming Concepts

### What I Understand

**Variables and Mutability**
- Variables are immutable by default (`let x = 5`)
- Use `mut` to make a variable mutable (`let mut y = 5`)
- Constants are always immutable and require type annotations (`const MAX_POINTS: u32 = 100_000`)
- Shadowing allows you to reuse variable names to transform values while maintaining immutability

**Data Types**
- Rust is statically typed — all types must be known at compile time
- Scalar types: integers (`i8`, `i32`, `u64`), floats (`f32`, `f64`), booleans, characters
- Compound types:
  - Tuples (can hold multiple types)
  - Arrays (fixed size, same type)
- Type annotations are needed when the compiler can’t infer the type (`let guess: u32`)

**Control Flow**
- `if` expressions require boolean conditions (no automatic type coercion like in JS)
- `if` can return values:
  ```rust
  let number = if condition { 5 } else { 6 };
````

* Loops:

  * `loop` — infinite loop
  * `while` — runs as long as condition is true
  * `for` — used to iterate over collections
* Use `break` to exit loops and `continue` to skip to the next iteration
* Loop labels (`'outer: loop`) help break or continue specific nested loops

###  What I Don't Understand

* **Integer overflow behavior**
  Why does it behave differently in debug vs release mode?

* **Loop labels syntax**
  What are real-world examples of when to use `'outer: loop`?

* **Range syntax details**
  What’s the exact difference between `1..4` and `1..=4`, and when is each appropriate?



## Chapter 4: Understanding Ownership

### What I Understand

>  I haven’t read Chapter 4 yet

### What I Don't Understand

Since I haven’t studied Chapter 4 yet, **everything** is unclear, including:

* **What is ownership?**

* **How does borrowing work?**

* **What are references?**


* **Stack vs Heap**


* **Memory safety**

* **Lifetimes**

* **The slice Types**

