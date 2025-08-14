#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
}

struct ShippingBox {
    dimensions: (f64, f64, f64),
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(dimensions: (f64, f64, f64), weight: f64, color: Color) -> Self {
        ShippingBox {
            dimensions,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        let (length, width, height) = self.dimensions;
        println!("Shipping Box Characteristics:");
        println!("Dimensions: {} x {} x {} cm", length, width, height);
        println!("Weight: {} kg", self.weight);
        println!("Color: {:?}", self.color);
    }
}

fn main() {
    let my_box = ShippingBox::new((30.0, 20.0, 15.0), 5.5, Color::Blue);
    my_box.print_characteristics();
}
