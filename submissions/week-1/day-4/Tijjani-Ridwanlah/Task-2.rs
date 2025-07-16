pub struct ShippingBox {
    pub dimensions: (f32, f32, f32),
    pub weight: f32,
    pub color: BoxColor,
}

pub enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
}

impl ShippingBox {
    pub fn new(dimensions: (f32, f32, f32), weight: f32, color: BoxColor) -> Self {
        ShippingBox {
            dimensions,
            weight,
            color,
        }
    }
    pub fn display_dimensions(&self) {
        println!("Dimensions: {:?}", self.dimensions);
    }

    pub fn display_weight(&self) {
        println!("Weight: {} kg", self.weight);
    }

    pub fn display_color(&self) {
        println!("Color: {:?}", self.color);
    }
}
