# Everything about Ownership, References and Borrowing in RUST

## Ownership
The concept of borrowing, ownership and references all boils down to how Rust manages memory. In a computer system, we have the RAM, and the SSD and all the processes that are being run are handle by the RAM. Unlike how javascript handles memory management using Gabbage collector.

**Before we go further, let's look at the stack and the heap....**

**Stack**- We can compare stack to a stacks of plates which follows the LIFO (Last In, First Out)- that is the last items inserted will be removed first. This is fast for allocation and deallocation as it's mostly use for primitive data types and for data where there size is known at compile time.
**Heap**- On the other hand, heap is used for data that can grow at runtime, like strings.

Ownership Rules
There can only be an owner for a value, and when the owner get out of scope the value will be dropped. 

Variable Scope
Variable scope are part of the code where a variable is valid and usable. For example, if I declare a variable `let s = "world";` inside a block, it only exists and can be used within that block. Once the block ends, the variable is no longer valid which mean it's “goes out of scope.”

**The complex types (String)**
Having go through all the previous basics, understanding memory management requires working with more complex data types, especially ones that live on the heap. That’s where `String` comes in. It’s a perfect example to learn how Rust handles ownership.

A typical string is the string literals `let s = "hello";`These are stored in the binary and are immutable. They're useful for static text, what if I need a dynamic input? Rust solves this with the `String` type  `let s = string::from("hello);`

Here, the `String::from()` function creates a new string stored on the heap. This means the size can grow and change at runtime and is also mutable, unlike fixed string literals.

**Memory and Allocation**
String literals are fast because they’re stored in the binary and can’t change.  
But when I use `String`, Rust allocates memory on the heap at runtime to allow growth.  Instead of using a garbage collector, Rust automatically frees that memory when the variable goes out of scope using the `drop` function.

Variables and data Interacting with move
While working with variables in Rust, I learned that assigning simple values like integers just copies the data. For instance, `let x = 5; let y = x;` gives me two separate variables that both hold 5, no issue there.

But with complex types like `String`, it’s a different thing. When I assign one `String` variable to another say `let s2 = s1;`  Rust doesn’t clone the actual string data on the heap. It only copies the pointer, length, and capacity from the stack. That means both `s1` and `s2` would point to the same heap data, and if both tried to free it, it could cause a problem.

To prevent that, Rust automatically invalidates `s1` after the move. So I can’t use `s1` after it’s moved to `s2`. 

## References and Borrowing

While learning about ownership, I first ran into a small issue when working with functions. Initially, when I passed a `String` into a function like `calculate_length`, ownership was transferred into the function, meaning I could no longer use the original variable afterward unless I returned it back as part of a tuple. 

The concept of  **references**, which let me borrow a value without taking ownership. By using `&r1` instead of just `r1`, I was able to pass a reference to the function. So inside `calculate_length`, the function could still read the data (like its length), but `r1` remained valid in the main function afterward.
Example code
```rust
fn main() {
    let r1 = String::from("hello");

    let len = calculate_length(&r1);

    println!("The length of '{r1}' is {len}.");
}

fn calculate_length(r: &String) -> usize {
    r.len()
}
```
So the action of creating a reference is borrowing. However, we're not allowed to modify reference.

What if we want to mutate a reference?

**Mutable References**
While learning about mutable references in Rust, I discovered how to safely modify borrowed data. By using `&mut`, I can pass a mutable reference to a function, allowing it to change the original value. However, Rust enforces strict rules: you can’t have more than one mutable reference at a time, and you can't mix mutable and immutable references to the same variable simultaneously.

**Dangling References**
A common issue in other languages where a reference points to memory that’s already been freed. Rust prevents this at compile time. The example in the book shows a function that returns a reference to a local `String`, like this:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
```
But Rust refused to compile it. The error explained that the code was trying to return a reference to a value (`s`) that would be dropped at the end of the function which will point to an invalid memeory.

What solves this?
Transfered ownership out of the function.
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```










 




