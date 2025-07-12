#[derive(Debug)]
enum BoxColor {
    Red,
    Blue,
    Green,
    Brown,
    White,
}

struct ShippingBox {
    dimensions: (f64, f64, f64), 
    weight: f64,                 
    color: BoxColor,
}

impl ShippingBox {
    fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            dimensions: (length, width, height),
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("Shipping Box Characteristics:");
        println!("Dimensions: {:.1}\" x {:.1}\" x {:.1}\"", 
                 self.dimensions.0, self.dimensions.1, self.dimensions.2);
        println!("Weight: {:.1} lbs", self.weight);
        println!("Color: {:?}", self.color);
    }
}

fn main() {
    let box1 = ShippingBox::new(12.0, 8.0, 6.0, 2.5, BoxColor::Brown);
    box1.print_characteristics();
    
    println!();
    
    let box2 = ShippingBox::new(24.0, 18.0, 12.0, 10.0, BoxColor::Blue);
    box2.print_characteristics();
}
