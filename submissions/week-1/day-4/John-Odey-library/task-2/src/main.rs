#[derive(Debug)]
enum Color {
    Brown,
    Red,
}

struct ShippingBox {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(width: f64, height: f64, depth: f64, weight: f64, color: Color) -> Self {
        Self {
            width,
            height,
            depth,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("Dimensions: {}x{}x{}", self.width, self.height, self.depth);
        println!("Weight: {} kg", self.weight);
        println!("Color: {:?}", self.color);
    }
}

fn main() {
    let box1 = ShippingBox::new(10.5, 8.0, 4.2, 5.1, Color::Brown);
    println!("--- Box 1 ---");
    box1.print_characteristics();

    let box2 = ShippingBox::new(20.0, 15.0, 10.0, 12.5, Color::Red);
    println!("\n--- Box 2 ---");
    box2.print_characteristics();
}
