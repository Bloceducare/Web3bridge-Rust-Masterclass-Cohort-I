# Chapter 3 continuation Summary

## Functions

### What I learned

**Functions:**
- Functions are declared using the `fn` keyword
- Rust uses snake_case for function and variable names
- Functions can be defined before or after their usage in the code
- The main function is the entry point of Rust programs

- Functions can accept parameters (special variables in function signature)
- Parameter types must be explicitly declared
- Multiple parameters are separated by commas
- Arguments are the concrete values passed to parameters

- **Statements** are Instructions that perform actions but don't return values
- **Expressions** evaluate to a resultant value
- Function calls, macros, and code blocks are expressions
- Expressions don't end with semicolons; adding one turns them into statements

- Functions can return values using the `->` syntax to declare return type
- The last expression in a function body is implicitly returned
- Use `return` keyword for early returns
- Adding semicolon to the last expression causes compilation error, because the expression then turns into a statement.


##  Comments

### What I learned

**Comment Style:**
- Single-line comments start with `//`
- Comments continue until end of line
- Multi-line comments require `//` on each line


##  Control Flow

### What I learned

Decision making is done using `if` expressions and repetition with loops i.e `loop`, `while`, and `for` loop statements

### If statement

- While using `if` statements, the condition must be a `bool` - Rust won't automatically convert other types
- Multiple conditions can be handled with `else if`
- `if` is an expression, so it can be used in `let` statements

### Loops

**Three Types of Loops and Their Differences:**

#### 1. `loop` - Infinite Loop
- **Purpose**: Creates an infinite loop that runs forever until explicitly stopped
- **When to use**: When you don't know how many times you need to loop
- **How to exit**: Must use `break` to exit
- **Special feature**: Can return values using `break value`


#### 2. `while` - Conditional Loop  
- **Purpose**: Runs while a specific condition remains true
- **When to use**: When you know the condition to check but not the exact number of iterations
- **How to exit**: Automatically exits when condition becomes false
- **Limitation**: Can be error-prone when accessing array elements (potential out-of-bounds)


#### 3. `for` - Collection/Range Iteration
- **Purpose**: Iterates over collections, arrays, or ranges
- **When to use**: When you want to loop through each item in a collection or a known range
- **How to exit**: Automatically exits when all elements are processed
- **Advantages**: Safest and most common loop type, prevents out-of-bounds errors


**Loop Control (applies to all types):**
- `break` - Exit the current loop
- `continue` - Skip to the next iteration

### What I do not understand.
Nuthin at the the time of writing.

