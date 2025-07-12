enum BoxColor {
    Red,
    Blue,
    Green,
}

struct ShippingBox {
    length: f64,
    width: f64,
    height: f64,
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> ShippingBox {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_charact(&self) {
        println!("Box Characteristics:");
        println!("  Dimensions: {:.1} x {:.1} x {:.1} cm", self.length, self.width, self.height);
        println!("  Weight: {:.1} kg", self.weight);
        
        let color = match self.color {
            BoxColor::Red => "Red",
            BoxColor::Blue => "Blue",
            BoxColor::Green => "Green",
        };
        println!("  Color: {}", color);
    }
}

fn main() {
    let ship_box = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Green);
    

    ship_box.print_charact();
}