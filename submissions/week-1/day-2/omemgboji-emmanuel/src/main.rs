fn main() {
    let a: i32 = 10;
    let b: i32 = 5;
    let float_a: f32 = 10.5;
    let float_b: f32 = 5.5;

    println!("Addition of integers: 10 + 5 = {}", add(a, b));
    println!(
        "Addition of floating point numbers: 10.5 + 5.5 = {}",
        float_add(float_a, float_b)
    );

    println!("Subtraction of integers: 10 - 5 = {}", substract(a, b));
    println!("Substraction of integers: 5 - 7 = {}", substract(5, 7));
    println!(
        "Subtraction of floating point numbers: 10.5 - 5.5 = {}",
        float_substract(float_a, float_b)
    );

    println!("Multiplication of integers: 10 * 5 = {}", multiply(a, b));

    println!("Division of integers: 10 / 5 = {}", divide(a, b));

    println!("Modulus of integers: 10 % 5 = {}", modulus(a, b));
    println!("Modulus of integers: 10 % 3 = {}", modulus(10, 3));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn float_add(a: f32, b: f32) -> f32 {
    a + b
}

fn substract(a: i32, b: i32) -> i32 {
    a - b
}

fn float_substract(a: f32, b: f32) -> f32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn modulus(a: i32, b: i32) -> i32 {
    a % b
}
