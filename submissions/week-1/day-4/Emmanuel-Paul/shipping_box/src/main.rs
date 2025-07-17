// Shipping Box Example with impl in Rust

#[derive(Debug)]
enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Debug)]
struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: BoxColor,
}

impl ShippingBox {
    // Associated function to create a new ShippingBox
    fn new(length: f32, width: f32, height: f32, weight: f32, color: BoxColor) -> Self {
        Self {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    // Method to print the characteristics
    fn print_characteristics(&self) {
        println!(
            "Dimensions: {} x {} x {}",
            self.length, self.width, self.height
        );
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
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 5.5, BoxColor::Blue);
    let box2 = ShippingBox::new(50.0, 40.0, 25.0, 10.0, BoxColor::Red);

    println!("Shipping Box 1:");
    box1.print_characteristics();
    println!("---");
    println!("Shipping Box 2:");
    box2.print_characteristics();
}
