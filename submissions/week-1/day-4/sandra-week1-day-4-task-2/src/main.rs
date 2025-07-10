#[derive(Debug)]
enum BoxColor {
    Red,
    Blue,
    Green,
    Brown,
    White,
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
    
    fn new(length: f32, width: f32, height: f32, weight: f32, color: BoxColor) -> Self {
        Self {
            length,
            width,
            height,
            weight,
            color,
        }
    }

   
    fn print_characteristics(&self) {
        println!("=== Shipping Box Characteristics ===");
        println!("Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        println!("Color: {:?}", self.color);
        println!("Volume: {:.2} cubic cm", self.calculate_volume());
    }

    // Additional method to calculate volume
    fn calculate_volume(&self) -> f32 {
        self.length * self.width * self.height
    }
}

fn main() {
  
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Brown);
    
    
    let box2 = ShippingBox::new(50.0, 40.0, 30.0, 5.8, BoxColor::Blue);

    box1.print_characteristics();
    println!();
    box2.print_characteristics();
}
