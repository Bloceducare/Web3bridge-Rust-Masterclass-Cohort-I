// Declare enum outside of main
enum BoxColor {
    Red,
    Blue,
    Green,
}

// Declare struct outside of main
struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: BoxColor,
}

// Implement methods outside of main
impl ShippingBox {
    fn new(length: f32, width: f32, height: f32, weight: f32, color: BoxColor) -> ShippingBox {
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
        println!("Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        match self.color {
            BoxColor::Red => println!("Color: Red"),
            BoxColor::Blue => println!("Color: Blue"),
            BoxColor::Green => println!("Color: Green"),
        }
    }
}

fn main() {
    // Now you can create an instance and use it inside main
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Blue);
    box1.print_characteristics();
}
