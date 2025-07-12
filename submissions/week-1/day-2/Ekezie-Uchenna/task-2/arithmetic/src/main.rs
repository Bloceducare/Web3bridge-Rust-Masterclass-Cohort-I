
use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter first number:");
    io::stdin().read_line(&mut input)
      .expect("Failed to read the first number");
    let a: i32 = input.trim().parse()
        .expect("Please enter a valid integer for the first number");
    

    input.clear();
    println!("Enter second number:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read the second number");
    let b: i32 = input.trim().parse()
        .expect("Please enter a valid integer for the second number");

    println!("Addition: {} + {} = {}", a, b, addition(a, b));
    println!("Subtraction: {} - {} = {}", a, b, subtraction(a, b));
    println!("Multiplication: {} * {} = {}", a, b, multiplication(a, b));
    println!("Division: {} / {} = {}", a, b, division(a, b));
}
fn addition(a: i32, b: i32) -> i32 {
    a + b
}
fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}
fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}
fn division(a: i32, b: i32) -> f32 {
    if b == 0 {
        println!("Error: Division by zero is not allowed.");
        return 0.0;
    }
    a as f32 / b as f32
}



