## Rust Book Summary — Chapter 3 & 4

### Chapter 3: Functions and Control Flow

- Learnt about functions and how they are defined using `fn`.
- Control flow includes:
  - `if`, `else`, and `else if` for conditional logic.
   
- Understood the difference between:
  - **Statements**: instructions that perform actions (e.g., `let x = 5;`).
  - **Expressions**: return a value and can be part of statements.
- Control flow includes:
  - `if`, `else if`, `else` for conditional logic.
  - Loops:
    - `loop`: runs forever unless we explicitly break out.
    - `while`: runs while a condition is true.
    - `for`: used to loop over a range or collection.


### Chapter 4: Ownership, Referencing and Borrowing

- Every value in Rust has an **owner**.
- Rust uses **stack and heap**:
  - Stack: stores fixed-size data known at compile time.
  - Heap: stores dynamic data (e.g., `String`).
- `String` is stored on the heap and is growable.
- String literals are stored on the stack and are immutable.

#### Ownership Rules

- When a value is moved (e.g. `let b = s;`), `s` is no longer valid.
- To keep using a variable after moving, we use `.clone()` to deeply copy the data.
- Rust prevents multiple ownership to avoid memory duplication.

#### References and Borrowing

- References are like pointers that do not own the data.
- References are stored on the stack and point to data (usually on the heap).
- We can borrow data using `&` (immutable) or `&mut` (mutable).

#### Why References Solve Ownership Challenges

 References were introduced precisely to solve the "ownership transfer headache" while maintaining memory safety. Here's how:


#### Memory Management

- Rust automatically allocates memory when a function is called and deallocates it when the function ends.
- The `drop` function is called when a variable goes out of scope to free memory.
- Using references helps prevent multiple copies of the same data in memory.

