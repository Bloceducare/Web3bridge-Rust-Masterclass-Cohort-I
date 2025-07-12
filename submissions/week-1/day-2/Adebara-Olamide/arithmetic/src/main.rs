pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}
pub fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 { Err("Can not divide by zero".to_string()) } else { Ok(a / b) }
}

fn main() {
    let sum = addition(5, 3);
    let difference = subtraction(10, 4);
    let product = multiplication(6, 7);
    let division = divide(8.0, 0.0);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Division: {:?}", division);
    // Uncomment the following lines to handle division result
    // match division {
    //     Ok(result) => println!("Division Result: {}", result),
    //     Err(e) => println!("Error: {}", e),
    // }
}
