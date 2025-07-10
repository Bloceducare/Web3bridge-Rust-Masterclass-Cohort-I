fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let sum = add(25, 4);
    println!("Addition: The sum of 25 + 4 = {}", sum);

    let difference = subtract(10, 2);
    println!("Subtraction: The difference between 10 - 2 = {}", difference);

    let product = multiply(2,2);
    println!("Multiplication: The product of 2 * 2 = {}", product);

    let quotient = divide(20, 4);
    println!("Division: The quotient of 20/4 = {}", quotient);
}