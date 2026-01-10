// fn main() {
    
//     // addition
//     let a = 9;
//     let b = 6;
//     let sum = a + b;

//     println!("The sum is {}", sum);

//     // subtraction
//     let a = 10;
//     let b = 7;
//     let diff = a - b;

//     println!("The diff is {}", diff);

//     // multiplication
//     let a = 8.8;
//     let b = 2.3;
//     let prod = a * b;

//     println!("The prod is {}", prod);

//     // division
//     let a = 19.9;
//     let b = 5.3;
//     let quot = a / b;

//     println!("The quot is {}", quot);
// }


// fn main() {
//     let a = 20.0;
//     let b = 4.0;


//     println!("Arithmetic Operations in Rust");
   

//     println!("Adding: {} + {} = {}", a, b, add(a, b));
//     println!("Subtracting: {} - {} = {}", a, b, subtract(a, b));
//     println!("Multiplying: {} * {} = {}", a, b, multiply(a, b));
//     println!("Dividing: {} / {} = {}", a, b, divide(a, b));
// }

// // sum function
// fn add(x: f64, y: f64) -> f64 {
//     x + y
// }

// // difference function
// fn subtract(x: f64, y: f64) -> f64 {
//     x - y
// }

// // product function
// fn multiply(x: f64, y: f64) -> f64 {
//     x * y
// }

// // division function
// fn divide(x: f64, y: f64) -> f64 {
//     if y == 0.0 {
//         println!("⚠️ Cannot divide by zero. Returning 0.");
//         0.0
//     } else {
//         x / y
//     }
// }


// fn main() {
//   let x: fsize = 2.0;
//   println!("{x}");
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
//     println!("The value of x is: {x}");
//     println!("The value of z is: {z}");
// }

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }


// fn main() {
//   let message = "The temperature today is:";
//   let x = [message, 100];
//   println!("{} {}", x[0], x[1]);
// }


fn main() {
  let t = ([1; 2], [3; 4]);
  let (a, b) = t;
  println!("{}", a[0] + t.1[0]); 
}