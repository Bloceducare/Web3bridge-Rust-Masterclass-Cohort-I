enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
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
        let color_str = match self.color {
            BoxColor::Red => "Red",
            BoxColor::Blue => "Blue",
            BoxColor::Green => "Green",
            BoxColor::Yellow => "Yellow",
        };
        println!("Color: {}", color_str);
    }
}

fn main() {
    let my_box = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Green);
    my_box.print_characteristics();
}
