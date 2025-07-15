// ## Requirements

// - Print the characteristics of a shipping box
// - Must include dimensions, weight, and color

// ## Notes

// - Use a struct to encapsulate the box characteristics
// - Use an enum for the box color
// - Implement functionality on the box struct to create a new box
// - Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
    Black,
    White,
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

    fn display_characteristics(&self) {
        println!("Shipping Box Characteristics:");
        println!("Dimensions: length {} x width {} x height {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        match self.color {
            BoxColor::Red => println!("Color: Red"),
            BoxColor::Blue => println!("Color: Blue"),
            BoxColor::Green => println!("Color: Green"),
            BoxColor::Yellow => println!("Color: Yellow"),
            BoxColor::Black => println!("Color: Black"),
            BoxColor::White => println!("Color: White"),
        }
    }
}

fn main() {
    let box1 = ShippingBox::new(10.0, 5.0, 8.0, 2.5, BoxColor::Blue);
    box1.display_characteristics();
    let box2 = ShippingBox::new(12.0, 6.0, 10.0, 3.0, BoxColor::Red);
    box2.display_characteristics();
    let box3 = ShippingBox::new(15.0, 7.0, 12.0, 4.0, BoxColor::Green);
    box3.display_characteristics();
}
