# Chapter 5: Using Structs to Structure Related Data

## Defining and Instantiating Structs

* **Structs** are custom data types grouping related fields under one name. Each field has a name and a type (unlike tuples, which only have positional fields). For example:

  ```rust
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  ```

  This defines a `User` type with four fields.
* **Creating an instance:** Use the struct name with `{ field: value, ... }`. The order of fields doesn’t matter, but all must be specified (unless using shorthand or update syntax). For example:

  ```rust
  let user1 = User {
      active: true,
      username: String::from("someusername123"),
      email: String::from("someone@example.com"),
      sign_in_count: 1,
  };
  ```

  This initializes a `User` with the given values.
* **Accessing and mutating fields:** Use dot notation: e.g. `user1.email` to read. To change a field, the entire instance must be `mut`, e.g. `user1.email = String::from("new@example.com");`. Rust does not support making individual fields mutable; the whole struct must be mutable.
* **Field init shorthand:** If a constructor’s parameter names match field names, you can omit repetition. Instead of `email: email`, write just `email`. For example:

  ```rust
  fn build_user(email: String, username: String) -> User {
      User { email, username, active: true, sign_in_count: 1 }
  }
  ```

  This is equivalent to explicitly `email: email` etc. (See ).
* **Struct update syntax:** To create a new instance by copying most fields from another, use `..other`. For example:

  ```rust
  let user2 = User {
      email: String::from("another@example.com"),
      ..user1
  };
  ```

  This sets `email` explicitly and fills the remaining fields from `user1`. Note that using this moves any non-`Copy` fields from `user1`.
* **Tuple structs:** A tuple struct has named type but unnamed fields. For example:

  ```rust
  struct Color(i32, i32, i32);
  let black = Color(0, 0, 0);
  ```

  Each tuple struct is a distinct type even if its fields share types. You access fields by index or by destructuring: e.g. `let Color(r, g, b) = black;`.
* **Unit-like structs:** A struct with no fields is declared with a semicolon, e.g. `struct AlwaysEqual;`. It behaves like `()`. Unit-like structs are useful for implementing traits on some type without data.
* **Ownership in structs:** Struct fields can own data. Using owned types (like `String`) means the struct instance owns that data for its lifetime. You *can* store references in structs, but then you must specify lifetimes explicitly. By default examples use owned data for simplicity.

## Example: Using Structs (Rectangle Area)

* A simple program refactors from separate width/height variables, to a tuple, to a struct. Using a struct makes code clearer by naming fields. For example:

  ```rust
  struct Rectangle {
      width: u32,
      height: u32,
  }

  fn main() {
      let rect1 = Rectangle { width: 30, height: 50 };
      println!(
          "The area of the rectangle is {} square pixels.",
          area(&rect1)
      );
  }
  fn area(rectangle: &Rectangle) -> u32 {
      rectangle.width * rectangle.height
  }
  ```

  Here `area` takes an immutable borrow `&Rectangle` to avoid taking ownership. This makes it explicit that `width` and `height` are related fields of one object.
* **Derived traits for debugging:** To print structs for debugging, derive the `Debug` trait. For example:

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }
  ```

  Then `println!("{rect1:?}", rect1);` will print field values. Adding `{:?}` or `{:#?}` enables developer-friendly output. The `dbg!(&rect1)` macro can also be used: it prints the expression and its value with file/line info.

## Method Syntax

* **Defining methods:** Methods are functions inside an `impl` block with `self` as the first parameter. For example:

  ```rust
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }
  ```

  Calling `rect1.area()` computes the area. The first parameter can be `self` (by value), `&self`, or `&mut self`, controlling ownership/borrowing. In the above, `&self` borrows the instance for reading.
* Methods with multiple parameters work like normal functions after `self`. For example:

  ```rust
  impl Rectangle {
      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }
  ```

  This returns `true` if `other` fits in `self`.
* **Associated functions:** These are functions in `impl` without `self`, called with `Type::func()`. Commonly used as constructors. Example:

  ```rust
  impl Rectangle {
      fn square(size: u32) -> Self {
          Self { width: size, height: size }
      }
  }
  let sq = Rectangle::square(3);
  ```

  Here `Self` is an alias for `Rectangle` inside the `impl`.
* **Name collisions:** A method can have the same name as a field. For example, if `Rectangle` has a field `width`, you could define a method `fn width(&self) -> bool { self.width > 0 }`. The call `rect.width()` invokes the method, whereas `rect.width` accesses the field.
* **Automatic referencing/dereferencing:** When calling methods (`instance.method()`), Rust automatically borrows or dereferences `instance` as needed to match the method’s signature.
* **Multiple `impl` blocks:** You can split methods into multiple `impl Type { }` blocks. All methods defined for a type (across blocks) are available normally.
* **Summary:** Structs define custom types with named fields, and `impl` blocks define associated functions and methods for those types. Methods organize behavior of the type and use `self` to refer to the instance.

# Chapter 6: Enums and Pattern Matching

## Defining Enums

* An **enum** defines a type by listing its possible variants. For example:

  ```rust
  enum IpAddrKind {
      V4,
      V6,
  }
  ```

  Here `IpAddrKind` can be either `V4` or `V6`. Each variant is namespaced under the enum (use `IpAddrKind::V4`).
* **Creating instances:** Use the enum name and variant, e.g. `let kind = IpAddrKind::V4;`. Both `IpAddrKind::V4` and `IpAddrKind::V6` have type `IpAddrKind`.
* **Data in variants:** Enum variants can hold data. Instead of a separate struct with a kind field, you can attach data directly to each variant. For example:

  ```rust
  enum IpAddr {
      V4(String),
      V6(String),
  }
  let home = IpAddr::V4(String::from("127.0.0.1"));
  ```

  Each variant name also becomes a constructor function (e.g. `IpAddr::V4(String)`).
* **Different data per variant:** Variants can contain different types or numbers of fields. For example:

  ```rust
  enum IpAddr {
      V4(u8, u8, u8, u8),
      V6(String),
  }
  let ip1 = IpAddr::V4(127,0,0,1);
  let ip2 = IpAddr::V6(String::from("::1"));
  ```

  This flexibility (unlike a single struct) lets each variant have its own data.
* **Standard library example:** The standard library’s `IpAddr` enum uses two variants wrapping different structs:

  ```rust
  enum IpAddr {
      V4(Ipv4Addr),
      V6(Ipv6Addr),
  }
  ```

  This shows that enum variants can even hold other structs.
* **Complex enums:** Enums can mix unit variants, tuple variants, and struct variants. For example:

  ```rust
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }
  ```

  This single `Message` type covers four cases that would otherwise require four different types. Using one enum makes handling them more ergonomic.
* **Methods on enums:** Enums can have methods via `impl` just like structs. For example:

  ```rust
  impl Message {
      fn call(&self) { /* ... */ }
  }
  let m = Message::Write(String::from("hello"));
  m.call();
  ```

  The method body can use `self` to access the variant’s data.
* **The `Option<T>` enum:** A ubiquitous enum in Rust, defined as:

  ```rust
  enum Option<T> {
      None,
      Some(T),
  }
  ```

  It represents “some value or nothing”. `Option<T>` is in the prelude, and its variants `Some` and `None` are available without qualification. Using `Option<T>` forces the programmer to handle the case of absence explicitly, eliminating null errors: an `Option<i8>` cannot be used as an `i8` without unwrapping or matching.

## Pattern Matching with `match`

* **`match` basics:** The `match` construct compares a value against patterns and executes the code for the first matching arm. It is like a switch statement but more powerful: patterns can include literals, enum variants, tuples, wildcards, etc. Every possible case must be covered, or a catch-all `_` arm must be provided.
* **Example:** Given an enum of US coins, return their value in cents:

  ```rust
  enum Coin { Penny, Nickel, Dime, Quarter }
  fn value_in_cents(coin: Coin) -> u8 {
      match coin {
          Coin::Penny => 1,
          Coin::Nickel => 5,
          Coin::Dime   => 10,
          Coin::Quarter=> 25,
      }
  }
  ```

  Each arm `pattern => expr` returns a value (here the coin’s value) for the entire `match` expression. If you need multiple lines in a branch, use braces and ensure the block’s last expression is returned.
* **Binding values:** Patterns can bind parts of a value. For example, if `Quarter` held a state:

  ```rust
  enum UsState { Alabama, Alaska, /*...*/ }
  enum Coin { Quarter(UsState), /*...*/ }
  match coin {
      Coin::Quarter(state) => println!("State quarter from {:?}", state),
      _ => (),
  }
  ```

  Here `state` is bound to the inner `UsState` when `coin` is a `Quarter`. You can then use `state` in the arm’s code.
* **Wildcards:** The `_` pattern matches anything and ignores it. It’s used to cover “all other cases”. In the above example, `_ => ()` does nothing for non-quarters. (In a full `match`, a `_` arm or matching all variants is required for exhaustiveness.)
* **Control flow:** The code for the matching arm is executed and then `match` exits. The value of that arm’s expression is the value of the `match` expression (if used in an assignment or return).
* **Exhaustiveness:** The compiler checks that all possible cases are handled. If you omit a case (and omit `_`), it’s a compile-time error.

## Concise Control Flow with `if let` and `let ... else`

* **`if let`:** A shorthand for matching one pattern and ignoring others. Instead of writing a full `match` with one useful arm and `_`, you can write:

  ```rust
  if let Some(max) = config_max {
      println!("The maximum is configured to be {max}");
  }
  ```

  This runs the block only if `config_max` matches `Some(max)`. Under the hood it’s equivalent to matching and ignoring the `_` case.
* The trade-off is that `if let` does *not* enforce exhaustiveness. It’s best for cases where you care about one variant and can ignore all others. For example, the earlier match:

  ```rust
  match config_max {
      Some(max) => println!("Max is {max}"),
      _ => (),
  }
  ```

  can be written as above `if let Some(max) = config_max { ... }`.
* **`else` with `if let`:** You can pair `if let` with `else` to handle the non-matching case. For example:

  ```rust
  if let Coin::Quarter(state) = coin {
      println!("State quarter from {state:?}!");
  } else {
      count += 1;
  }
  ```

  is equivalent to a `match` with a `_` arm.
* **`let ... else` (Rust 1.65+):** This form allows early return on non-match while keeping the matching path unindented. For example:

  ```rust
  fn describe_state_quarter(coin: Coin) -> Option<String> {
      let Coin::Quarter(state) = coin else { return None; };
      // Now use `state` knowing it’s a Quarter
      if state.existed_in(1900) {
          Some(format!("{state:?} is pretty old!"))
      } else {
          Some(format!("{state:?} is relatively new."))
      }
  }
  ```

  The `else` block must exit (e.g. `return`). This keeps the “happy path” in the main flow.
* **Summary:** Use `match` for exhaustively handling all enum variants (with powerful patterns), and use `if let` (or `let ... else`) for concise handling of a single pattern when appropriate.

**Sources:** The above concepts are summarized from *The Rust Programming Language*, chapters 5 and 6. Each bullet point is supported by the cited text.
