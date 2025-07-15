enum BoxColor {
    Blue,
    Yellow,
}

struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f64,
    color: BoxColor,
}
impl ShippingBox {
    fn new(length: f32, width: f32, height: f32, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }


    fn print_characteristics(&self) {
        println!("Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        println!(
            "Color: {}",
            match self.color {
                BoxColor::Blue => "Blue",
                BoxColor::Yellow => "Yellow",
            }
        );
    }
}
fn main() {
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Blue);
    let box2 = ShippingBox::new(25.0, 15.0, 10.0, 1.5, BoxColor::Yellow);

    println!("Box 1 Characteristics:");
    box1.print_characteristics();

    println!("\nBox 2 Characteristics:");
    box2.print_characteristics();
}
