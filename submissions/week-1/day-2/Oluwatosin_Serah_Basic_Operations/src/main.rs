fn addition(x:f64, y:f64) -> f64{
    x + y
}

fn subtract(x:f64, y:f64) -> f64{
    x - y
}

fn multiply(x:f64, y:f64) -> f64{
    x * y
}

fn divide(x:f64, y:f64) -> f64{
    x / y
}

fn main(){
    let x = 32.0;
    let y = 4.0;

    println!("The result of addition is: {}", addition(x, y));
    println!("The result of subtraction is: {}", subtract(x, y));
    println!("The result of multiplication is: {}", multiply(x, y));
    println!("The result of division is: {}", divide(x, y));
}
