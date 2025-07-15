#[derive(Debug)] // Needed for {:?} formatting
enum BoxColor {
    Red,
    Blue,
    Green,
    Brown,
}

struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: BoxColor,
}

impl ShippingBox {
    fn new(length: f32, width: f32, height: f32, weight: f32, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!(
            "Box Dimensions: {} x {} x {}",
            self.length, self.width, self.height
        );
        println!("Weight: {} kg", self.weight);
        println!("Color: {:?}", self.color); // or use match if you prefer
    }
}

fn main() {
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 5.5, BoxColor::Blue);
    box1.print_characteristics();
}
