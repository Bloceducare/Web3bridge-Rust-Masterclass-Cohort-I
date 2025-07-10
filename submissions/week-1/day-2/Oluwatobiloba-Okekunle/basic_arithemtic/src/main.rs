fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    a / b
}

fn main() {
    // Test values
    let num1 = 20;
    let num2 = 5;
    let num1_f64 = 20.0;
    let num2_f64 = 5.0;

    // Perform and print all operations
    println!("Basic Arithmetic Operations:");
    println!("---------------------------");
    println!("Addition: {} + {} = {}", num1, num2, add(num1, num2));
    println!("Subtraction: {} - {} = {}", num1, num2, subtract(num1, num2));
    println!("Multiplication: {} * {} = {}", num1, num2, multiply(num1, num2));
    println!("Division: {} / {} = {}", num1_f64, num2_f64, divide(num1_f64, num2_f64));
}
