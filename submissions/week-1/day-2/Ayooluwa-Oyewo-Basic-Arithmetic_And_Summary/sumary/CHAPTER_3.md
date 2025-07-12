
## What I Understand from Chapter 3

From my study of Chapter 3, I learned several foundational things about how Rust handles variables, data, and control flow.

### 🔐 Variables and Mutability

In Rust, variables are **immutable by default**, which means once you assign a value to a variable, you can’t change it unless you explicitly make it mutable using `mut`. This helps make programs more predictable and safe.

```rust
let x = 5;
x = 6; // error

let mut x = 5;
x = 6; // works
```

There are also **constants** using `const`, which are always immutable and require type annotations.

I also learned about **shadowing**, which lets you use the same variable name again, even to change its type, without using `mut`.

```rust
let spaces = "   ";
let spaces = spaces.len(); // now an integer
```

### 📚 Data Types

Rust is statically typed, so every variable has a known type at compile time. There are **scalar types** like integers, floats, booleans, and characters — and **compound types** like tuples and arrays.

I saw that Rust gives us different sizes of integers, both signed and unsigned (e.g., `i32`, `u64`, etc.), and has default types (`i32` for integers and `f64` for floats).

Also, trying to access an array out of bounds causes a runtime panic.

### 🛠 Functions

Functions are declared with `fn`, and every parameter must have a type. Rust functions can return values either implicitly (with no semicolon) or explicitly using `return`.

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

I also understood the difference between **statements** and **expressions** — expressions return a value, but statements don’t.

### 💬 Comments

Rust uses `//` for single-line comments and supports multi-line ones by stacking `//`.

```rust
// This is a comment
let x = 5; // inline comment
```

### 🔀 Control Flow

Rust has `if`, `loop`, `while`, and `for` for control flow.

* **if** requires conditions to be boolean and can be used as an expression.
* **loop** runs forever unless you use `break`.
* You can also label loops when working with nested ones.
* **while** keeps looping while a condition is true.
* **for** is very useful and safe for iterating over arrays and ranges.

Example:

```rust
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!");
```

---

### Summary

Overall, I now understand how Rust encourages writing safe and predictable code by making immutability the default, requiring explicit types, and giving powerful tools like pattern matching, strict control flow, and clear memory safety rules through things like ownership (which comes in the next chapter). These concepts, though new, make a lot of sense once you get used to them.
