pub enum Color {
    Red,
    Blue,
    Green,
}

pub struct ShippingBox {
    width: f32,
    height: f32,
    depth: f32,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    pub(crate) fn new(width: f32, height: f32, depth: f32, weight: f32, color: Color) -> Self {
        ShippingBox { width, height, depth, weight, color }
    }

    pub(crate) fn print_characteristics(&self) {
        let color_name = match self.color {
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Green => "Green",
        };
        println!("Dimensions: {}x{}x{}", self.width, self.height, self.depth);
        println!("Weight: {}", self.weight);
        println!("Color: {}", color_name);
    }
}
