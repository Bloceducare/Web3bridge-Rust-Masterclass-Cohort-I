use std::io;

fn main() {
    println!("==============================");
    println!("        PALASA Calculator");
    println!("==============================");

    // Get first number from user
    println!("Enter the first number:");
    let a = read_number();

    // Get second number from user
    println!("Enter the second number:");
    let b = read_number();

    println!("\nPerforming operations on {} and {}...\n", a, b);

    let sum = add(a, b);
    println!("➕ Addition:       {} + {} = {}", a, b, sum);

    let difference = subtract(a, b);
    println!("➖ Subtraction:    {} - {} = {}", a, b, difference);

    let product = multiply(a, b);
    println!("✖️ Multiplication: {} * {} = {}", a, b, product);

    let result = divide(a, b);
    match result {
        Some(value) => println!("➗ Division:       {} / {} = {}", a, b, value),
        None => println!("➗ Division:       Cannot divide by zero."),
    }

    println!("\n==============================");
}

// Reads a number (f64) from user input
fn read_number() -> f64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse::<f64>().expect("Invalid number entered")
}

// ➕ Add
fn add(x: f64, y: f64) -> f64 {
    x + y
}

// ➖ Subtract
fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

// ✖️ Multiply
fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

// ➗ Divide (handles divide-by-zero)
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}
