fn main() {
    let a = 10;
    let b = 3;
    
    println!("=== Arithmetic Operations ===");
    println!("Numbers: {} and {}", a, b);
    println!("");
    
    let sum = add(a, b);
    println!("Addition: {} + {} = {}", a, b, sum);
    
    let difference = subtract(a, b);
    println!("Subtraction: {} - {} = {}", a, b, difference);
    
    let product = multiply(a, b);
    println!("Multiplication: {} * {} = {}", a, b, product);
    
    let quotient = divide(a, b);
    println!("Division: {} / {} = {}", a, b, quotient);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn divide(x: i32, y: i32) -> f64 {
    x as f64 / y as f64  // Convert to f64 for decimal division
}