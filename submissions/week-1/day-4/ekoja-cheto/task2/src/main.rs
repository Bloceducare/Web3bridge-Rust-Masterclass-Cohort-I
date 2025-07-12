
enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
    Custom(String),
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
        println!("Shipping Box Characteristics:");
        println!("Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        print!("Color: ");
        match &self.color {
            BoxColor::Red => println!("Red"),
            BoxColor::Blue => println!("Blue"),
            BoxColor::Green => println!("Green"),
            BoxColor::Yellow => println!("Yellow"),
            BoxColor::Custom(name) => println!("Custom ({})", name),
        }
    }
}


fn main() {
    
    let my_box = ShippingBox::new(30.0, 20.0, 15.0, 5.5, BoxColor::Green);

    
    my_box.print_characteristics();
}
