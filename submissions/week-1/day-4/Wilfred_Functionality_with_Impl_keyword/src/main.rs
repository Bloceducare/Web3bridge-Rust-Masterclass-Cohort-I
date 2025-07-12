enum Color {
    Red,
    Blue,
    Green,
}

struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(length: f32, width: f32, height: f32, weight: f32, color: Color) -> Self {
        Self {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_details(&self) {
        println!("Dimensions: {} x {} x {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        match self.color {
            Color::Red => println!("Color: Red"),
            Color::Blue => println!("Color: Blue"),
            Color::Green => println!("Color: Green"),
        }
    }
}

fn main() {
    let my_box = ShippingBox::new(12.0, 8.0, 4.5, 3.2, Color::Red);
    my_box.print_details();
}
