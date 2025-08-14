#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct Box {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: Color,
}

impl Box {
    fn new(length: f32, width: f32, height: f32, weight: f32, color: Color) -> Box {
        Box {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_box_ch(&self) {
        println!("Box Characteristics:");
        println!("Dimensions: length {}, width {}, height {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        println!("Color: {:?}", self.color);
    }
}

fn main() {
    let shipping_box = Box::new(30.0, 20.0, 15.0, 2.5, Color::Blue);
    shipping_box.print_box_ch();
    
}
