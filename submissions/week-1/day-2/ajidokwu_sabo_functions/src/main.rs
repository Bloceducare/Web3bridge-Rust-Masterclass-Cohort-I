fn main() {
    let sum = sum(5, 6);
    println!("The sum is {}", sum);

    let sub = sub(15, 6);
    println!("The difference is {}", sub);

    let div = div(36, 6);
    println!("The sudivisionm is {}", div);

    let prod = prod(5, 6);
    println!("The product is {}", prod);

}

fn sum(num_1: i32, num_2: i32) -> i32{
    let answer = num_1 + num_2;
   return answer;
}

fn sub(num_1: i32, num_2: i32) -> i32{
    let answer = num_1 - num_2;
   return answer;
}

fn div(num_1: i32, num_2: i32) -> i32{
    let answer = num_1 / num_2;
   return answer;
}

fn prod(num_1: i32, num_2: i32) -> i32{
    let answer = num_1 * num_2;
   return answer;
}