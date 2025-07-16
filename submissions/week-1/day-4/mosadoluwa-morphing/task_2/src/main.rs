// Enum for box color
enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
}

// Struct for shipping box characteristics
struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: BoxColor,
}

// Implement functionality for ShippingBox
impl ShippingBox {
    // Create a new box
    fn new(length: f32, width: f32, height: f32, weight: f32, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    // Print the characteristics of the box
    fn print_characteristics(&self) {
        println!("Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        print!("Color: ");
        match self.color {
            BoxColor::Red => println!("Red"),
            BoxColor::Blue => println!("Blue"),
            BoxColor::Green => println!("Green"),
            BoxColor::Yellow => println!("Yellow"),
        }
    }
}

fn main() {
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Blue);
    box1.print_characteristics();
}