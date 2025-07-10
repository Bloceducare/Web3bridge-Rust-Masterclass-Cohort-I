// Adds two integers and returns the result.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Subtracts the second integer from the first and returns the result.
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// Multiplies two integers and returns the result.
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Divides the first integer by the second and returns the result as a floating-point number.
// Returns None if division by zero is attempted.
fn divide(a: i32, b: i32) -> Option<f32> {
    if b == 0 {
        None
    } else {
        Some(a as f32 / b as f32)
    }
}

// Entry point of the program. Demonstrates the usage of arithmetic functions.
fn main() {
    let a = 10;
    let b = 5;

    println!("Addition: {} + {} = {}", a, b, add(a, b));
    println!("Subtraction: {} - {} = {}", a, b, subtract(a, b));
    println!("Multiplication: {} * {} = {}", a, b, multiply(a, b));

    match divide(a, b) {
        Some(result) => println!("Division: {} / {} = {}", a, b, result),
        None => println!("Division: {} / {} = Error (division by zero)", a, b),
    }
}
