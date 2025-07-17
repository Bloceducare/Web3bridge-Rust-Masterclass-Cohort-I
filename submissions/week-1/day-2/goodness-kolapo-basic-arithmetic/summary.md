
# Chapter 3.4: Comments

We use `//` for single-line comments, `/* ... */` for multi-line, and `///` for doc comments (which show up in docs)

---

# Chapter 3.5: Control Flow

Rust uses `if` statements for making decisions, and three kinds of loops: `loop` (runs forever unless you break), `while` (runs while a condition is true), and `for` (great for going through arrays and ranges)


# Chapter 4 Summary: Ownership in Rust

 Chapter 4 of Rust is about **ownership**. Rust is strict about who owns what in the codebase,

- Every value in Rust has an owner. Only one owner at a time.

```rust
fn main() {
    let a = String::from("owned");
    let b = a;
    // a is now invalid, b owns the value
}
```

- When the owner goes out of scope, Rust just drops (deletes) the value. 

```rust
fn main() {
    {
        let s = String::from("bye");
    }
    // s is dropped here
}
```

- If you try to use something after it’s been moved (ownership transferred), Rust will shout at you (compiler error).

```rust
fn main() {
    let s1 = String::from("move");
    let s2 = s1;
    println!("{}", s1);
}
```

- Borrowing is when you use a value without taking ownership. You do this with & (immutable borrow).

```rust
fn main() {
    let s = String::from("borrow");
    print_length(&s);
    println!("{}", s);
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}
```

- Lending is when you let someone else use your value, but you still own it(no ownership chhhange). You can lend with & (immutable) or &mut (mutable), so it applies to both mutable and immutable variables.

```rust
fn main() {
    let mut s = String::from("lend");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str("ed");
}
```


