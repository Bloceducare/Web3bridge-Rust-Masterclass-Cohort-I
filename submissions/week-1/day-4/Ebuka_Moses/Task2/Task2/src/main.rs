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
        println!("Dimensions: {} x {} x {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        println!("Color: {}", self.get_color());
    }

    fn get_color(&self) -> &str {
        match self.color {
            BoxColor::Red => "Red",
            BoxColor::Blue => "Blue",
            BoxColor::Green => "Green",
            BoxColor::Yellow => "Yellow",
        }
    }
}

fn main() {

    let box_red = ShippingBox::new(30.0, 20.0, 15.0, 5.5, BoxColor::Red);
    let box_blue = ShippingBox::new(25.0, 18.0, 12.0, 4.0, BoxColor::Blue);
    let box_green = ShippingBox::new(35.0, 22.0, 17.0, 6.0, BoxColor::Green);
    let box_yellow = ShippingBox::new(28.0, 19.0, 14.0, 4.8, BoxColor::Yellow);

    box_red.print_characteristics();
    println!("---");
    box_blue.print_characteristics();
    println!("---");
    box_green.print_characteristics();
    println!("---");
    box_yellow.print_characteristics();
}
