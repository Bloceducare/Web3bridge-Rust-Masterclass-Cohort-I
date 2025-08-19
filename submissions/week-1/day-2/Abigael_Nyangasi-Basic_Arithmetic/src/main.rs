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
    if b == 0.0 {
        println!("Warning: Division by zero!");
        f64::NAN
    } else {
        a / b
    }
}

fn main() {
    let x = 10.0;
    let y = 5.0;

    println!("=== Basic Arithmetic Operations ===");
    println!("Add:       {} + {} = {}", x, y, add(x, y));
    println!("Subtract:  {} - {} = {}", x, y, subtract(x, y));
    println!("Multiply:  {} * {} = {}", x, y, multiply(x, y));
    println!("Divide:    {} / {} = {}", x, y, divide(x, y));
}
