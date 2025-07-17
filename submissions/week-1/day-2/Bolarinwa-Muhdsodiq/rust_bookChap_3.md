# Chapter 3: Rust Book

## Common Programming Concepts

---

### Variables and Mutability

Variables are immutable by default, which gives you the advantage of static typing. Once declared, you cannot reassign a value to an immutable variable unless it is mutable.

To set up a project, use:

```rust
Cargo new
```

Example of an immutable variable:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // This will cause an error
    println!("The value of x is: {x}");
}
```

The code above will throw an error because `x` is immutable and cannot be reassigned. To make a variable mutable, use the `mut` keyword:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

---

### Constants

Constants are values attached to a name that cannot be changed. You cannot use the `mut` keyword with constants.

Example:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

---

### Shadowing

Shadowing allows you to redeclare a variable with the same name using the `let` keyword. This is different from making a variable mutable.

```rust
fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

Example:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

---

### Data Types

#### Scalar Types

Scalar types represent a single value. The four primary scalar types are:

- Integers
- Floating-point numbers
- Booleans
- Characters

**Integer Types:**

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

- Signed and unsigned refer to whether it’s possible for the number to be negative (signed) or only positive (unsigned).
- Each signed variant can store numbers from −(2ⁿ⁻¹) to 2ⁿ⁻¹−1.

Numbers can also be written with a suffix to indicate the type:

| Number Literal | Example     |
| -------------- | ----------- |
| Decimal        | 98_222      |
| Hex            | 0xff        |
| Octal          | 0o77        |
| Binary         | 0b1111_0000 |
| Byte (u8)      | b'A'        |

- **Integer Overflow** happens when the value exceeds the range of the type.

**Floating-Point Types:**

- `f32` and `f64` are the two floating-point types.

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
```

**Numeric Operations:**

```rust
fn main() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
}
```

**Boolean Type:**

- Can be either `true` or `false`.

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```

**Character Type:**

- The `char` type is four bytes and represents a Unicode Scalar Value.
- Use single quotes for `char` literals.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

---

### Compound Types

**Tuple Type:**

- Tuples group together values of different types with a fixed length.
- Written as a comma-separated list inside parentheses.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

Accessing tuple values:

- **Destructuring:**

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}
```

- **Dot notation:**

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

**Array Type:**

- Arrays store a collection of the same type with a fixed size.
- Useful for data allocated on the stack.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}

let a: [i32; 5] = [1, 2, 3, 4, 5];

let months = [
    "January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"
];
```

Accessing array elements by index:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

- The value at index `[0]` is the first value. If you try to access an index outside the array, you get an out-of-bounds panic.

---

### Functions

Functions are a prominent feature in Rust. The `main` function is the entry point of a Rust application. Functions are defined with the `fn` keyword, followed by parentheses (which may contain parameters), and curly braces.

```rust
fn main() {
    println!("Hello, world!");
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

**Parameters:**

- Parameters are specified in the parentheses and are part of the function signature.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

**Statements and Expressions:**

- Statements perform actions and do not return a value.
- Expressions evaluate to a value.

Example of a statement:

```rust
fn main() {
    let y = 6;
}
```

Example of an expression:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}
```

**Functions with Return Values:**

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");
}
```

---

### Comments

Comments are used to add notes or explanations to your code for easier understanding.

Example:

```rust
// hello, world
```

---

### Control Flow

**If Statement:**

- Used to compare variables and make decisions.

```rust
fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

You can compare booleans to numbers.

**If-Else Statement:**

- Used to check multiple conditions in sequence.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
```
