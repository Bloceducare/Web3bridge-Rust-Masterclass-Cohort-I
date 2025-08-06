#[derive(Debug)]
enum BoxColor {
    Brown,
    Blue,
}

    struct ShippingBox {
    length: f32,    
    width: f32,     
    height: f32,    
    weight: f32,    
    color: BoxColor,
}

impl ShippingBox {
    fn new(length: f32, width: f32, height: f32, weight: f32, color: BoxColor) -> ShippingBox {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("Box Characteristics:");
        println!("  Length: {} cm", self.length);
        println!("  Width: {} cm", self.width);
        println!("  Height: {} cm", self.height);
        println!("  Weight: {} kg", self.weight);
        println!("  Color: {:?}", self.color);
        println!(); 
    }
}

fn main() {
    let my_box = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Brown);
    
    my_box.print_characteristics();
    
    let another_box = ShippingBox::new(50.0, 40.0, 30.0, 5.0, BoxColor::Blue);
    
    another_box.print_characteristics();
}