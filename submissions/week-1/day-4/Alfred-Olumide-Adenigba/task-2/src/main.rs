// Enum
#[derive(Debug, Clone, Copy)]
enum BoxColor {
    Brown,
    White,
    Blue,
    Red,
    Green,
    Black,
}

// Struct
#[derive(Debug)]
struct ShippingBox {
    length: f64,
    width: f64,
    height: f64,
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {

    fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("=== Shipping Box Characteristics ===");
        println!("Dimensions: {:.2} x {:.2} x {:.2} cm", self.length, self.width, self.height);
        println!("Weight: {:.2} kg", self.weight);
        println!("Color: {:?}", self.color);
    }

}

// Implementation
impl std::fmt::Display for BoxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxColor::Brown => write!(f, "Brown"),
            BoxColor::White => write!(f, "White"),
            BoxColor::Blue => write!(f, "Blue"),
            BoxColor::Red => write!(f, "Red"),
            BoxColor::Green => write!(f, "Green"),
            BoxColor::Black => write!(f, "Black"),
        }
    }
}

fn main() {
    
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Brown);
    box1.print_characteristics();
    
    println!(); 
    
    let box2 = ShippingBox::new(45.0, 35.0, 25.0, 5.2, BoxColor::Blue);
    box2.print_characteristics();
    
}