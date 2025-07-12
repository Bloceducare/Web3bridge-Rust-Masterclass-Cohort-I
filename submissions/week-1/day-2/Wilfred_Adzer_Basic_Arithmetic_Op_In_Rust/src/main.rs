
fn main() {

    let a = 10;
    let b = 5;




    println!("Addition: {a} + {b} = {}", add(a,b));
    println!("Subtraction: {a} + {b} = {}", sub(a,b));
    println!("Multiplication: {a} + {b} = {}", mul(a,b));
    println!("Division: {a} + {b} = {}", div(a,b));


}


pub fn add(a: u64, b:u64)-> u64 {
    a + b
}


pub fn sub(a:u64, b:u64) -> u64 {
    a - b
}


pub fn mul(a:u64, b:u64) -> u64 {
    a * b
}


pub fn div(a:u64, b:u64)-> u64 {
    a/b
}
