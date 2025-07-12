#[derive(Debug, Clone)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct ShippingBox {
    dimensions: (u64, u64, u64), // length, width, height
    weight: u64,
    color: Color,
}

impl ShippingBox {
    fn new(dimensions: (u64, u64, u64), weight: u64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) -> ShippingBox {
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {:?}", self.weight);
        println!("Color: {:?}", self.color.clone());

        ShippingBox {
            dimensions: self.dimensions,
            weight: self.weight,
            color: self.color.clone(),
        }
    }
}

fn main() {
    let shipping_box = ShippingBox::new((10, 10, 10), 10, Color::Red);
    shipping_box.print_characteristics();
}
