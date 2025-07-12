enum BoxColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
}

struct ShippingBox {
    width: f32,
    height: f32,
    depth: f32,
    weight: f32,
    color: BoxColor,
}

impl ShippingBox {
    fn new(width: f32, height: f32, depth: f32, weight: f32, color: BoxColor) -> Self {
        ShippingBox {
            width,
            height,
            depth,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!(
            "Dimensions: {} x {} x {} cm",
            self.width, self.height, self.depth
        );
        println!("Weight: {:.2} kg", self.weight);
        println!(
            "Color: {}",
            match self.color {
                BoxColor::Red => "Red",
                BoxColor::Orange => "Orange",
                BoxColor::Yellow => "Yellow",
                BoxColor::Green => "Green",
                BoxColor::Blue => "Blue",
            }
        );
    }
}

fn main() {
    let box1 = ShippingBox::new(40.0, 25.0, 15.0, 3.2, BoxColor::Orange);
    let box2 = ShippingBox::new(55.5, 30.0, 20.0, 6.8, BoxColor::Blue);

    box1.print_characteristics();
    println!("---");
    box2.print_characteristics();
}
