enum BoxColor {
    Red,
    Blue,
    Green,
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
        println!("Dimensions: {} x {} x {}", self.width, self.height, self.depth);
        println!("Weight: {} kg", self.weight);
        print!("Color: ");
        match self.color {
            BoxColor::Red => println!("Red"),
            BoxColor::Blue => println!("Blue"),
            BoxColor::Green => println!("Green"),
        }
    }
}

fn main() {
    let my_box = ShippingBox::new(10.0, 5.0, 8.0, 2.5, BoxColor::Green);
    my_box.print_characteristics();
}
