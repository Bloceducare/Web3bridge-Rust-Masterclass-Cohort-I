fn main() {
    let x = 20;
    let y = 4;

    let sum = add(x, y);
    let difference = subtract(x, y);
    let product = multiply(x, y);
    let quotient = divide(x, y);

    println!("=== Arithmetic Operations ===");
    println!("Addition: {} + {} = {}", x, y, sum);
    println!("Subtraction: {} - {} = {}", x, y, difference);
    println!("Multiplication: {} * {} = {}", x, y, product);
    println!("Division: {} / {} = {:.2}", x, y, quotient);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> f64 {
    if b != 0 {
        a as f64 / b as f64
    } else {
        panic!("Cannot divide by zero");
    }
}
