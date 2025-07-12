
enum Color {
    White,
    Blue,
}

struct Box {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: Color,
}


impl Box {
    
    fn new() -> Box {
        Box {
            length: 20.0,
            width: 10.0,
            height: 5.0,
            weight: 6.0,
            color: Color::White,
        }
    }

    
    fn info(&self) {
        println!("The Length is : {}", self.length);
        println!("The Width is : {}", self.width);
        println!("The Height is : {}", self.height);
        println!("The Weight is: {}", self.weight);

        match self.color {
            Color::White => println!("Color: White"),
            Color::Blue => println!("Color: Blue"),
        }
    }
}

fn main() {
    let my_box = Box::new();
    my_box.print_info();
}
