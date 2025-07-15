mod addition;
mod subtraction;
mod multiplication;
mod division; 



fn main() {
    // This program performs basic arithmetic operations: addition, subtraction, multiplication, and division.
    let num1: i32 = 10;
    let num2: i32 = 2;

    let sum = addition::addition(num1, num2);
    let difference = subtraction::subtraction(num1, num2);
    let product = multiplication::multiplication(num1, num2);
    let quotient = division::division(num1, num2); 

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {:?}", quotient);




}
