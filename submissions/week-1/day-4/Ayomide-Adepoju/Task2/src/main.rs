struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: BoxColor,
}

enum BoxColor {
    White,
    Red,
    Green,
    Yellow,
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
        println!("Dimensions: {} x {} x {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        println!("Color: {}", match self.color {
            BoxColor::White => "White",
            BoxColor::Red => "Red",
            BoxColor::Green => "Green",
            BoxColor::Yellow => "Yellow",
        });
    }
}

fn main() {
    let my_box = ShippingBox::new(10.0, 5.0, 3.0, 2.5, BoxColor::Blue);

    my_box.print_characteristics();
}
