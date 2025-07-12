

// Adds two numbers
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Subtracts second number from first
fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

// Multiplies two numbers
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// Divides first number by second
fn divide(x: i32, y: i32) -> i32 {
    x / y
}



fn main() {
    let a = 20;
    let b = 5;

    println!("--- Basic Arithmetic ---");
    println!("Operands: a = {}, b = {}", a, b);
    println!();

    let sum = add(a, b);
    let difference = subtract(a, b);
    let product = multiply(a, b);
    let quotient = divide(a, b);

    println!("Results:");
    println!("Addition       : {} + {} = {}", a, b, sum);
    println!("Subtraction    : {} - {} = {}", a, b, difference);
    println!("Multiplication : {} * {} = {}", a, b, product);
    println!("Division       : {} / {} = {}", a, b, quotient);
}

