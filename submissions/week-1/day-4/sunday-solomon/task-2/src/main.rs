#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct ShippingBox {
    width: f32,
    height: f32,
    depth: f32,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(width: f32, height: f32, depth: f32, weight: f32, color: Color) -> Self {
        Self {
            width,
            height,
            depth,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("Dimensions: {} x {} x {}", self.width, self.height, self.depth);
        println!("Weight: {} kg", self.weight);
        println!("Color: {:?}", self.color);
        println!();
    }
}

fn main() {
    let box1 = ShippingBox::new(10.0, 5.0, 8.0, 3.5, Color::Blue);
    box1.print_characteristics();
    let box1 = ShippingBox::new(10.0, 5.0, 8.0, 3.5, Color::Red);
    box1.print_characteristics();
    let box1 = ShippingBox::new(10.0, 5.0, 8.0, 3.5, Color::Green);
    box1.print_characteristics();
}
