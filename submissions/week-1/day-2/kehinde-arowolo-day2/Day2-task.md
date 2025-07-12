
````markdown

# Chapter 3 – What I Learned: Functions, Comments & Control Flow

##  Functions (My Understanding)
- I define a function using `fn` and give it a name.
- Every parameter needs a type - Rust doesn’t guess like some languages.
- If I want to return something, I use `->` and then the return type.
- I noticed that if I **don’t** add a semicolon at the end, Rust sees it as the return value .

```rust
fn multiply(x: i32, y: i32) -> i32 {
    x * y // returned automatically
}
```

---

##  Comments

- Comments in Rust use `//` — same as many other languages.

```rust
// Calculating the user's total score
```

---

##  Control Flow (How Rust Makes Decisions)

###  If Statements

- Rust doesn’t allow non-boolean conditions — that keeps things clean.
- I don’t need parentheses around the `if` condition (unlike other languages).
- I can use `if`, `else if`, and `else` for branching.

```rust
if temp > 30 {
    println!("Hot day");
} else {
    println!("Cool weather");
}
```

###  If as an Expression

- I can assign a value based on a condition using `if` directly inside a `let`.
- Both `if` and `else` blocks must return the **same type**.

```rust
let message = if is_logged_in { "Welcome" } else { "Please login" };
```

---

##  Loops (Repeating Stuff)

###  loop

- `loop` runs forever unless I use `break`.
- Useful for retries or waiting for something to happen.

```rust
loop {
    println!("Still running...");
    break;
}
```

###  while

- Works like in other languages — keeps running while a condition is true.

```rust
while count < 3 {
    println!("Counting...");
    count += 1;
}
```

###  for

- This is my favorite loop so far.
- Clean, readable, and great for working with ranges and collections.
- I can also reverse it using `.rev()`.

```rust
for i in 1..4 {
    println!("{}", i); // prints 1 to 3
}
```

```rust
for i in (1..4).rev() {
    println!("{}", i); // prints 3 to 1
}
```

---

## My Takeaway

- Rust is strict but smart. I now understand why it forces things like type annotations and booleans in `if`.
- The little things like returning without `return`, and using `if` in `let` bindings, make writing Rust feel expressive once I get used to it.
- Control flow in Rust feels safe and readable, which helps me write more predictable logic.




# Chapter 4 Summary - Ownership, Referencingmand slice in Rust

## Heap vs Stack
- The heap is **less organized** than the stack.
- When you put data on the heap, you **request** a certain amount of space.
- The memory allocator finds an **empty spot** big enough, marks it as **in use**, and returns a **pointer** (an address to that location).

---

## Ownership Rules
1. Each value in Rust has an **owner**.
2. There can only be **one owner at a time**.
3. When the owner goes **out of scope**, the value is **dropped**.

---

## Scope
-  **scope** is the range within a program for which an item is **valid**.

---

## String Literals vs String Type
- String literals are **convenient** but not suitable for every situation.
- We may want to use text that is **not known at compile time**.

---

## Stack and Heap
- **Integers** are simple values with a **fixed size**, stored on the **stack**.
- Example:
  ```rust
  let x = 5;
  let y = x;
  ```

- For `String`:
  ```rust
  let s1 = String::from("hello");
  let s2 = s1;
  ```

- A `String` is made up of:
  * A **pointer** to the memory,
  * A **length**,
  * A **capacity**.

- When a variable goes out of scope, Rust **automatically drops** and **cleans up** the memory for that variable.

---

## References and Borrowing

- A **reference** is like a pointer — an **address** used to access data stored elsewhere.
- The referenced data is **owned by another variable**.
- We reference a value when we **don’t want to take ownership**.

---

## Scalar vs Compound Types

- **Scalar types** (like integers) do **not come with ownership**.
- **Compound types** (like `String`) **come with ownership** and can be used by referencing.
- These are also stored on the **heap**.

---

## Dereferencing

- The **opposite** of referencing is **dereferencing**.
- It is done using the **dereference operator `*`**.

---

## Slices

- A **slice** lets you reference a **contiguous sequence of elements** in a collection without taking ownership.
- Commonly used with **strings** and **arrays**.

###  String Slices
```rust
let s = String::from("hello world");
let hello = &s[0..5]; // "hello"
let world = &s[6..];  // "world"
```

- Slices are of type `&str`, meaning they are **borrowed string slices**.
- They are references, so they follow the **borrowing rules**.

---

###  Array Slices
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3]; // contains elements 2 and 3
```

---

###  Slice Safety

- Rust checks that slice ranges are **valid at runtime**.
- If the range is out of bounds, the program will **panic**.

---

###  Why Use Slices?

- Avoids **copying data**.
- Lets you work with **parts of data** without taking ownership.
- Enforces **memory safety** while enabling **efficient access**.
````

---
