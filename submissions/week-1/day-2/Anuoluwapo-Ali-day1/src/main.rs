fn main() {
    let a = 25;
    let b = 10;
    let add = addition(a as f64, b as f64);
    let sub = substraction(a, b);
    let mult = multiplication(a, b);
    let div = division(a, b);

    println!("Addition of two floats: {}", add);
    println!("Subtraction of two ints: {}", sub);
    println!("Multiplication of two ints: {}", mult);
    println!("Division of two ints: {}", div);

}

fn addition(a: f64, b: f64) -> f64 {
    let result = a + b;
    result
}

fn substraction(a: u32, b: u32) -> u32 {
    let result = a - b;
    result
}

fn multiplication(a: u32, b: u32) -> u32 {
    let result = a * b;
    result
}

fn division(a: u32, b: u32) -> f64 {
    let result =  a as f64 / b as f64;
    result
}