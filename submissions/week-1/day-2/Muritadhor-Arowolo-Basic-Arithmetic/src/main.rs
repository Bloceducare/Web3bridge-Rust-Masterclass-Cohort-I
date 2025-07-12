fn main() {
    let a = 10;
    let b = 5;

    let sum = add(a, b);
    let difference = subtract(a, b);
    let quotient = divide(a, b);
    let product = multiply(a, b);

    println!("Performing basic arithmetic operations on {} and {}:", a, b);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Quotient: {}", quotient);
    println!("Product: {}", product);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
