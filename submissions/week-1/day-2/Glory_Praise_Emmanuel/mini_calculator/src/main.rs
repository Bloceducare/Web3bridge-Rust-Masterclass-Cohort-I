use std::io;

fn main() {
    println!("🧮 Welcome to your mini-calculator!");
    println!("Enter two numbers to add, divide, substract and divide!");
    
    let mut num1 = String::new();
    println!("Enter first number:");
    io::stdin()
     .read_line(&mut num1)
     .expect("Failed to read line");
    let num1:f64 = num1.trim().parse().expect("Please enter a valid number");

     let mut num2 = String::new();
    println!("Enter second number:");
    io::stdin()
     .read_line(&mut num2)
     .expect("Failed to read line");
    let num2:f64 = num2.trim().parse().expect("Please enter a valid number");


    let added = addition(num1, num2);
    let substracted = substraction(num1, num2);
    let divided = division(num1, num2);
    let multiplied = multiplication(num1, num2);


    println!("\nYou gave two numbers {} and {}", num1, num2);
    println!("Addition: {}", added);
    println!("Substraction: {}", substracted);
    println!("Division: {}", divided);
    println!("Multiplication: {}", multiplied);

}


fn addition(x:f64, y:f64) -> f64 {
   x + y
}

fn substraction(x:f64, y:f64) -> f64 {
   x - y
}

fn division(x:f64, y:f64) -> f64 {
    if y != 0.0 {
        x / y
    } else {
        println!("\n⚠️ Division error:  Cannot divide by zero. Returning 0.");
        0.0
    }
}

fn multiplication(x:f64, y:f64) -> f64 {
   x * y
}