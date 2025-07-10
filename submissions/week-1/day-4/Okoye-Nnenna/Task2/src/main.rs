#[derive(Debug)]  
enum BoxColor {
    Red,
    Blue,
    Green,
    Brown,
    White,
}

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
        println!("Shipping Box Characteristics:");
        println!("Dimensions: {}cm x {}cm x {}cm", self.length, self.width, self.height);
        println!("Weight: {}kg", self.weight);
        println!("Color: {:?}", self.color);
        println!("Volume: {:.2} cubic cm", self.length * self.width * self.height);
    }
}

fn main() {
    let shipping_box = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Brown); 
    shipping_box.print_characteristics();  
}