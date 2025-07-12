# Rust Book Assignment – Task 1

## Task

Study the remaining of **Chapter 3** and **all of Chapter 4** of *The Rust Programming Language* and:

- Summarize what you understand
- List what you do not understand

---

## 📖 Chapter 3 Summary

### 3.2 Data Types  
Rust has two main types of data:
- **Scalar types** are single values like numbers, booleans, or characters.
- **Compound types** can group values — like **tuples** (mix of types) or **arrays** (same type, fixed length).


### 3.3: Functions
Functions are blocks of code you can reuse. You write them using `fn`, give them inputs, and they can return results.
- Function syntax in Rust.
- Parameters and return values.
- Expressions vs. statements.
- Arrow syntax (`->`) for return types.

### 3.4: Comments
Comments help explain your code. Rust ignores them when running. You write them with `//`.
- Single-line and multiline commenting styles.

### 3.5: Control Flow
You use `if`, `else`, `loop`, `while`, and `for` to make decisions or repeat code. It’s how you control what your program does.
- `if` expressions and how they differ from other languages.
- `else if`, `else`, and conditional assignment using `let`.
- Loop types: `loop`, `while`, and `for`.

---

## 📖 Chapter 4 Summary

### 4.1: What is Ownership?
- Ownership is a set of rules that govern how a Rust program manages memory. 
- The ownership model gives you control over memory, is error free, gives you faster runtime, and a smaller program size but the trade off is the slower write time and learning curve(fighting with borrow checker).
- Understanding how the heap and stack data structure works. Stack used LiFo method, you can push and pop from the stack and  All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.The pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
- Ownership rules:
  1. Each value in Rust has a variable called its *owner*.
  2. There can only be one owner at a time.
  3. When the owner goes out of scope, the value is dropped.

  Deep copying of the stack data is done when a variable is assigned to another variable or cloning is done to deep copy the heap data entirely.This is for data like Strings without a known size at compile time, hence it is stored on the heap. For integars, these rules don't apply because they are stored on the stack.

- Refrences rules:
  1. At any given time, you can have either one mutable reference or any number of immutable references.
  2. References must always be valid.

### 4.2: References and Borrowing
Instead of copying, you can *borrow* values using `&`. You can borrow a value to read or change it but not both at the same time.
The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. 
- Borrowing with references (`&T`).
- Mutable references (`&mut T`) and restrictions (only one mutable reference or any number of immutable ones).


### 4.3: The Slice Type
A slice is just a part of something, like a part of a string or array. It lets you use data without owning or copying it.
- String slices (`&str`) and how they differ from full `String`.
- Array slices.

---

## Things I Do Not Understand Yet
- [ ] When do you choose a slice over a reference or clone?

---

