fn main() {
    let result = addition(4, 5);

    println!("The sum of 4 and 5: {}", result);

    let result = subtraction(4, 5);

    println!("The sum of 4 and 5: {}", result);

    let result = division(4, 5);

    println!("The sum of 4 and 5: {}", result);

    let result = multiplication(4, 5);

    println!("The sum of 4 and 5: {}", result);
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}

fn division(a: i32, b: i32) -> i32 {
    a / b
}

fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}
