fn main(){
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
        println!("Dimensions: {} x {} x {} cm", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        match self.color {
            BoxColor::Red => println!("Color: Red"),
            BoxColor::Blue => println!("Color: Blue"),
            BoxColor::Green => println!("Color: Green"),
        }
    }
}

}