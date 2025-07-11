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
    let a = 10;
    let b = 2;

    println!("Add: {}", add(a, b));
    println!("Subtract: {}", subtract(a, b));
    println!("Multiply: {}", multiply(a, b));
    println!("Divide: {}", divide(a, b));
}

