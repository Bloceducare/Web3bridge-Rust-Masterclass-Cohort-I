enum BoxColor {
    Red,
    Blue,
    Green,
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

    
    fn print_characteristics(&self) {
        println!("Box Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        match self.color {
            BoxColor::Red => println!("Color: Red"),
            BoxColor::Blue => println!("Color: Blue"),
            BoxColor::Green => println!("Color: Green"),
        }
    }
}


fn main() {
    let box1 = ShippingBox::new(50.0, 30.0, 20.0, 5.5, BoxColor::Blue);
    box1.print_characteristics();
}