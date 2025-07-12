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
    if b != 0.0 {
        a / b
    } else {
        println!("Cannot divide by zero!");
        0.0
    }
}

fn main() {
    let x = 10.0;
    let y = 5.0;

    println!("Arithmetic Operations on {} and {}", x, y);
    println!("--------------------------------------");
    println!("Addition Result: {}", add(x, y));
    println!("Subtraction Result: {}", subtract(x, y));
    println!("Multiplication Result: {}", multiply(x, y));
    println!("Division Result: {}", divide(x, y));
}
