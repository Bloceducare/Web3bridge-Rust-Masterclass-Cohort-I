use std::io;

fn main() {
    println!("Input the first number");

    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: f32 = x.trim().parse().expect("The value typed is not a number!");

    println!("Input the second number");

    let mut y = String::new();

    io::stdin().read_line(&mut y).expect("Failed to read line");

    let y: f32 = y.trim().parse().expect("The value typed is not a number!");


    add(x,y);
    substract(x,y);
    multiply(x,y);
    divide(x,y);
}

fn add(x:f32, y:f32){
    println!("The sum of of {x} and {y} is {}", x + y);
}

fn substract(x:f32, y:f32){
    println!("The difference of {x} and {y} is {}", x - y);
}

fn multiply(x:f32, y:f32){
    println!("The product of {x} and {y} is {}", x * y);
}

fn divide(x:f32, y:f32){
    println!("The quotient of {x} and {y} is {}", x / y);
}