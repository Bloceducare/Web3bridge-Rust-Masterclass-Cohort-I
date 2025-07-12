#[derive(Debug)]
enum BoxColor {
    Brown,
    White,
    Blue,
    Red,
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

    fn print_characteristics(&self) {
        println!("Shipping Box");
        println!("Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        println!("Color: {:?}", self.color);
        println!("Volume: {:.2} cubic cm", self.calculate_volume());
    }

    fn calculate_volume(&self) -> f64 {
        self.length * self.width * self.height
    }
}

fn main() {
    let small_box = ShippingBox::new(20.0, 15.0, 10.0, 1.5, BoxColor::Brown);
    let medium_box = ShippingBox::new(40.0, 30.0, 25.0, 3.2, BoxColor::White);
    let large_box = ShippingBox::new(60.0, 50.0, 40.0, 8.7, BoxColor::Blue);

    small_box.print_characteristics();
    println!();
    
    medium_box.print_characteristics();
    println!();
    
    large_box.print_characteristics();
}
