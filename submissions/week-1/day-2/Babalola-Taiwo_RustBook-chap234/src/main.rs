
use std::io;

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}

fn main() {
    println!("Please enter your first number:");

    let mut num1_input = String::new();

    io::stdin()
        .read_line(&mut num1_input)
        .expect("Failed to read line");

    let num1: f64 = num1_input
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    println!("Please enter the second number:");

    let mut num2_input = String::new();

    io::stdin()
        .read_line(&mut num2_input)
        .expect("Failed to read line");

    let num2: f64 = num2_input
        .trim()
        .parse()
        .expect("Invalid input. Please enter a valid number.");

    let sum_result = add(num1, num2);
    let difference_result = subtract(num1, num2);
    let product_result = multiply(num1, num2);
    let quotient_result = divide(num1, num2);

    println!("\n--- Basic Arithmetic Results for {} and {} ---", num1, num2);
    println!(" ");
    println!("Addition:       {}", sum_result);
    println!("Subtraction:    {}", difference_result);
    println!("Multiplication: {}", product_result);
    println!("Division:       {}", quotient_result);
}