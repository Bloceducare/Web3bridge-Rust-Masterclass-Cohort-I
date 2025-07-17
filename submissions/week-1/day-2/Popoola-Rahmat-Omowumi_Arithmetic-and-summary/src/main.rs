/// Adds 
fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts 
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides 
/// Returns `None` when attempting to divide by zero.
fn divide(a: f64, b: f64) -> Option<f64> {
    if b.abs() < f64::EPSILON {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
  
    let (x, y) = (12.0, 3.0);

    println!("================  ARITHMETIC RESULTS  =================\n");
    println!("Add       : {x} + {y} = {}", add(x, y));
    println!("Subtract  : {x} - {y} = {}", subtract(x, y));
    println!("Multiply  : {x} × {y} = {}", multiply(x, y));
    match divide(x, y) {
        Some(result) => println!("Divide    : {x} ÷ {y} = {result}"),
        None => println!("Divide    : attempted to divide by zero"),
    }
    println!("\n========================================================\n");
}

