fn main() {
    let a = 10.0;
    let b = 5.0;

    println!("Adding {} and {}: {}", a, b, add(a, b));
    println!("Subtracting {} from {}: {}", b, a, subtract(a, b));
    println!("Multiplying {} and {}: {}", a, b, multiply(a, b));
    
    match divide(a, b) {
        Some(result) => println!("Dividing {} by {}: {}", a, b, result),
        None => println!("Cannot divide {} by zero", b),
    }
}

// Add two numbers
fn add(x: f64, y: f64) -> f64 {
    x + y
}

// subtract two numbers
fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

// multiply two numbers
fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

// divide two numbers
fn divide(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}