use std::io;

fn main() {

    perform_arithmetic_operations()

}

fn perform_arithmetic_operations(){

    println!("Perform all four arithmetic operations on two numbers!!");

    println!("Input the first of the two numbers you want to perform arithmetics on");

    let mut x = String::new();
    
    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: isize = x.trim().parse().expect("The value typed in should be a number!");

    println!("Input the second of the two numbers you want to perform arithmetics on");

    let mut y = String::new();

    io::stdin().read_line(&mut y).expect("Failed to read line");

    let y: isize = y.trim().parse().expect("The value typed in should be a number!");

    println!("Here are the results of your arithmetic operation");

    println!("==============================================================================");

    add(x,y);

    println!("==============================================================================");

    substract(x,y);

    println!("==============================================================================");

    multiply(x,y);

    println!("==============================================================================");

    divide(x,y);

    println!("==============================================================================");

    println!("Arithmetics done sucessfully, Kindly re-run to input two new numbers, Thanks!");

}

fn add(x:isize, y:isize){

    println!("The result of the addition of {x} and {y} is {}", x + y);

}

fn substract(x:isize, y:isize){

    println!("The result of the subtraction of {x} and {y} is {}", x - y);

}

fn multiply(x:isize, y:isize){

    println!("The result of the multiplication of {x} and {y} is {}", x * y);

}

fn divide(x:isize, y:isize){

    println!("The result of the division of {x} and {y} is {}", x / y);

}
