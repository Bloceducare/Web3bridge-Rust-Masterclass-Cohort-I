fn main() {
   let a= 10;
   let b= 5;
   println!("add:{}",add(a,b));
   println!("substract:{}",subtract(a,b));
   println!("multiply:{}",multiply(a,b));
    println!("divide:{}",divide(a,b));
}





fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: i32, b: i32) -> i32 {
    a / b
}
