enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
}

struct ShippingBox {
    dimension: (f32, f32, f32),
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(self, dimension: (f32, f32, f32), weight: f32, color: Color) -> ShippingBox {
        ShippingBox {
            dimension,
            weight,
            color,
        }
    }

    fn display_dimension(&self) {
        println!(
            "Box Dimension: ({}, {}, {})",
            self.dimension.0, self.dimension.1, self.dimension.2
        );
    }

    fn display_weight(&self) {
        println!("Box Weight: {} kg", self.weight)
    }

    fn display_color(&self) {
        match self.color {
            Color::Red => println!("Box Color: Red"),
            Color::Green => println!("Box Color: Green"),
            Color::Blue => println!("Box Color: Blue"),
            Color::Yellow => println!("Box Color: Yellow"),
            Color::Black => println!("Box Color: Black"),
            Color::White => println!("Box Color: White"),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
