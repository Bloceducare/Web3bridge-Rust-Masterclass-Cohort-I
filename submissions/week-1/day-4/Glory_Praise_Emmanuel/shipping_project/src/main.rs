enum Color {
    Brown,
    Red,
    Blue,
    Green,
}

struct ShippingBox {
    dimension: i32,      // Since a shipping box is a cube, one side represents all dimensions
    weight: i32,
    color: Color,
    content: String,
}

impl ShippingBox {
    fn new(dimension: i32, weight: i32, color: Color, content: String) -> ShippingBox {
        ShippingBox {
            dimension,
            weight,
            color,
            content,
        }
    }

    fn print_box(&self) {
        println!("Shipping Box Info:");
        println!("Dimension:  {} cm × {} cm × {} cm ", self.dimension, self.dimension, self.dimension);
        println!("Weight: {} kg", self.weight);
        println!("Color: {}", self.get_color());
        println!("Content: {}", self.content);
    }

    // Helper function to convert enum to string
    fn get_color(&self) -> &str {
        match self.color {
            Color::Brown => "Brown",
            Color::Red => "Red",
            Color::Blue => "Blue",
            Color::Green => "Green",
        }
    }
}

fn main() {
    let box1 = ShippingBox::new(
        30,
        10,
        Color::Brown,
        String::from("Books"),
    );

    box1.print_box();
}