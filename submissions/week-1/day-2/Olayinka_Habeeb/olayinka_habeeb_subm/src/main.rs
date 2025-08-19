fn main(){
    let a = add_numbers(2.2,3.4);
    let b = sub_numbers(4,3);
    let c = mult_numbers(5,6);
    let d = div_numbers(4,2);
    
    println!("Output for Addition {}", a);
    println!("Output for Subtraction {}", b);
    println!("Output for Multiplication {}", c);
    println!("Output for Division {}", d);
}

fn add_numbers(x:f32, y: f32) -> f32{
    x + y
}

fn sub_numbers(x:u8, y:u8) -> u8{
    x - y
}
fn mult_numbers(x:u8, y:u8) -> u8{
    x * y
}

fn div_numbers(x:u8, y:u8) -> u8{
    x / y
}





