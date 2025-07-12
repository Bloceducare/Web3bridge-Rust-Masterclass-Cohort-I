#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Brown,
    White,
    Black,
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
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("Box Characteristics:");
        println!(
            "  Dimensions: {} x {} x {}",
            self.length, self.width, self.height
        );
        println!("  Weight: {}", self.weight);
        println!("  Color: {:?}", self.color);
    }
}

fn main() {
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 2.5, Color::Brown);
    box1.print_characteristics();
}
