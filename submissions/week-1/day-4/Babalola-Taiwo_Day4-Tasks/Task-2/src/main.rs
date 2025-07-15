enum Color {
    Red,
    Blue,
    Green,
}

struct ShippingBox {
    length_cm: u32,
    width_cm: u32,
    height_cm: u32,
    weight_kg: f64,
    color: Color,
}

impl ShippingBox {
    fn new(length_cm: u32, width_cm: u32, height_cm: u32, weight_kg: f64, color: Color) -> Self {
        Self {
            length_cm,
            width_cm,
            height_cm,
            weight_kg,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!(
            "Dimensions: {} x {} x {} cm",
            self.length_cm, self.width_cm, self.height_cm
        );
        println!("Weight: {} kg", self.weight_kg);
        let color_name = match self.color {
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Green => "Green",
        };
        println!("Color: {}", color_name);
    }
}

fn main() {
    let my_box = ShippingBox::new(30, 20, 10, 2.5, Color::Green);
    my_box.print_characteristics();
}
