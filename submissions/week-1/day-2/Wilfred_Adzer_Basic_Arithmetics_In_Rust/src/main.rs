mod Operation;

fn main() {

    let a = 10;
    let b = 5;




    println!("Addition: {a} + {b} = {}", Operation::add(a,b));
    println!("Subtraction: {a} + {b} = {}", Operation::sub(a,b));
    println!("Multiplication: {a} + {b} = {}", Operation::mul(a,b));
    println!("Division: {a} + {b} = {}", Operation::div(a,b));


}
