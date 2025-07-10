fn main() {
    const NUM_1: f32 = 25.5;
    const NUM_2: f32 = 50.25;

    addition(NUM_1, NUM_2);
    subtraction(NUM_1, NUM_2);
    multiplication(NUM_1, NUM_2);
    division(NUM_1, NUM_2);
}

fn addition(num_1: f32, num_2: f32) {
    println!("The sum of {num_1} and {num_2} is: {}", num_1 + num_2);
}

fn subtraction(num_1: f32, num_2: f32) {
    println!("The subtraction of {num_2} from {num_1} is: {}", num_1 - num_2);
}

fn multiplication(num_1: f32, num_2: f32) {
    println!("The multiplication of {num_1} by {num_2} is: {}", num_1 * num_2);
}

fn division(num_1: f32, num_2: f32) {
    println!("The division of {num_1} by {num_2} is: {}", num_1 / num_2);
}