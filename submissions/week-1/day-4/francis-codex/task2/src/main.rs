enum Color {
    Red,
    Blue,
    Green,
}

struct ShippingBox {
    width: f32,
    height: f32,
    depth: f32,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(width: f32, height: f32, depth: f32, weight: f32, color: Color) -> Self {
        ShippingBox { width, height, depth, weight, color }
    }

    fn print_characteristics(&self) {
        let color_name = match self.color {
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Green => "Green",
        };
        println!("Dimensions: {}x{}x{}", self.width, self.height, self.depth);
        println!("Weight: {}", self.weight);
        println!("Color: {}", color_name);
    }
}

fn main() {
    let box1 = ShippingBox::new(10.0, 5.0, 3.0, 2.5, Color::Red);
    box1.print_characteristics();
}