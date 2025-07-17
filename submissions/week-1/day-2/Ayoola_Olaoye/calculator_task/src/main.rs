fn main() {
    println!("Please enter first number:");
    let mut first_number = String::new();

    std::io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");
    let first_number: i32 = match first_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for first number");
            return;
        }
    };

    println!("Please enter second number:");
    let mut second_number = String::new();

    std::io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");
    let second_number: i32 = match second_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for second number");
            return;
        }
    };

    {
        add(first_number, second_number);
    }

    {
        subtract(first_number, second_number);
    }

    {
        multiply(first_number, second_number);
    }

    {
        divide(first_number, second_number);
    }
}

fn add(a: i32, b: i32) {
    println!("Addition of {} and {} is {}", a, b, a + b);
}

fn subtract(a: i32, b: i32) {
    println!("Subtraction of {} and {} is {}", a, b, a - b);
}

fn multiply(a: i32, b: i32) {
    println!("Multiplication of {} and {} is {}", a, b, a * b);
}

fn divide(a: i32, b: i32) {
    if b == 0 {
        println!("Cannot divide by zero");
        return;
    }
    println!("Division of {} and {} is {}", a, b, a / b);
}
