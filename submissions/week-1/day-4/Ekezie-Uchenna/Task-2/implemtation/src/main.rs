// # Task 2: Implementing Functionality with the `impl` Keyword

// ## Requirements

// - Print the characteristics of a shipping box
// - Must include dimensions, weight, and color

// ## Notes

// - Use a struct to encapsulate the box characteristics
// - Use an enum for the box color
// - Implement functionality on the box struct to create a new box
// - Implement functionality on the box struct to print the characteristics

// ---

// # Task 2: Implementing Functionality with the `impl` Keyword

struct ShippingBox {
    length: f64,
    width: f64,
    height: f64,
    weight: f64,
    color: BoxColor,
}
enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
}

impl ShippingBox {
    // Function to create a new ShippingBox
    fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    // Function to print the characteristics of the ShippingBox
    fn print_characteristics(&self) {
        println!("Shipping Box Characteristics:");
        println!(
            "Dimensions: {} x {} x {}",
            self.length, self.width, self.height
        );
        println!("Weight: {} kg", self.weight);
        match &self.color {
            BoxColor::Red => println!("Color: Red"),
            BoxColor::Blue => println!("Color: Blue"),
            BoxColor::Green => println!("Color: Green"),
            BoxColor::Yellow => println!("Color: Yellow"),
        }
    }
}

fn main() {
    // Create a new ShippingBox instance
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 5.0, BoxColor::Blue);

    // Print the characteristics of the ShippingBox
    box1.print_characteristics();
}
