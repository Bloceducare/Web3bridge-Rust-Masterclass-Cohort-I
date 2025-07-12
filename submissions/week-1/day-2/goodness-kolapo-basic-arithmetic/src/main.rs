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
        panic!("Cannot divide by zero");
    }
    a / b
}

fn main() {
    let x = 10.0;
    let y = 5.0;

    println!("Addition:    {} + {} = {}", x, y, add(x, y));
    println!("Subtraction: {} - {} = {}", x, y, subtract(x, y));
    println!("Multiplication: {} * {} = {}", x, y, multiply(x, y));
    println!("Division:    {} / {} = {}", x, y, divide(x, y));
}
