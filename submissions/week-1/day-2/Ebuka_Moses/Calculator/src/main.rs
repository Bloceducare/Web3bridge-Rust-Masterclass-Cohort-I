mod add;
mod subtract;
mod divide;
mod multiple;

fn main() {
    let a = 10.0;
    let b = 5.0;

    let sum = add::add(a, b);
    let diff = subtract::subtract(a, b);
    let div = divide::divide(a, b);
    let prod = multiple::multiply(a, b);

    println!("Performing basic arithmetic operations on {} and {}:", a, b);
    println!("Sum: {}", sum);
    println!("Diff: {}", diff);
    println!("Div: {}", div);
    println!("Prod: {}", prod);
}