fn add_two_numbers(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract_two_numbers(a: f64, b: f64) -> f64 {
    a - b
}


fn multiply_two_numbers(a: f64, b: f64) -> f64 {
    a * b
}

fn divide_two_numbers(a: f64, b: f64) -> f64 {
    a / b
}

fn main() {
    let a = 10.0;
    let b = 20.0;

    println!("Addition: {a} + {b} = {}", add_two_numbers(a, b));
    println!("Subtraction: {b} - {a} = {}", subtract_two_numbers(b, a));
    println!("Multiplication: {a} * {b} = {}", multiply_two_numbers(a, b));
    println!("Division: {a} / {b} = {}", divide_two_numbers(a, b));
}
