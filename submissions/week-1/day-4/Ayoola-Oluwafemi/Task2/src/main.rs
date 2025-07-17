
enum BoxColor {
    Red,
    Pink,
    Green,
    Purple,
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
        println!("Dimensions: {} x {} x {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        print!("Color: ");
        match self.color {
            BoxColor::Red => println!("Red"),
            BoxColor::Pink => println!("Pink"),
            BoxColor::Green => println!("Green"),
            BoxColor::Purple => println!("Purple"),
        }
    }
}

fn main() {
    let box1 = ShippingBox::new(10.0, 5.0, 3.0, 2.5, BoxColor::Green);
    box1.print_characteristics();
}
