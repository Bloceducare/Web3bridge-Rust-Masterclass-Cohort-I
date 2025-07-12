// fn addition(x:f64, y:f64) -> f64{
//     x + y
// }

// fn subtract(x:f64, y:f64) -> f64{
//     x - y
// }

// fn multiply(x:f64, y:f64) -> f64{
//     x * y
// }

// fn divide(x:f64, y:f64) -> f64{
//     x / y
// }

// fn main(){
//     let x = 32.0;
//     let y = 4.0;

//     println!("The result of addition is: {}", addition(x, y));
//     println!("The result of subtraction is: {}", subtract(x, y));
//     println!("The result of multiplication is: {}", multiply(x, y));
//     println!("The result of division is: {}", divide(x, y));
// }

use std::io;
use colored::*;

fn main() {
    println!("🧮 Enter the two numbers you want to perform basic arithemetic on");

    let num1: f64 = loop {
        println!("Enter the first number");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num, //break&retrun valid number
            Err(_) => {
                println!("{}", "Please enter a valid number!".red());
                continue;
            }
        };
    };

    let num2:f64 = loop {
        println!("Enter the second number");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("{}", "Please enter a valid number!".red());
                continue;
            }
        };
    };

    println!("You entered: num1 = {}, num2 = {}", num1, num2);
    println!("The result of {} + {} = {}", num1, num2, addition(num1, num2));
    println!("The result of {} - {} = {}", num1, num2, subtraction(num1, num2));
    println!("The result of {} / {} = {}", num1, num2, division(num1, num2));
    println!("The result of {} * {} = {}", num1, num2, multiplication(num1, num2));
}

fn addition(num1:f64, num2:f64) -> f64{
    num1 + num2
}

fn subtraction(num1:f64, num2:f64) -> f64{
    num1 - num2
}

fn division(num1:f64, num2:f64) -> f64{
    if num2 == 0.0{
        println!("{}", "Error: Cannot divide by zero. Please use a non-zero divisor. Returning 0.".red());
        num2
    } else{
        num1 / num2
    }
   
}

fn multiplication(num1:f64, num2:f64) -> f64{
    num1 * num2
}




