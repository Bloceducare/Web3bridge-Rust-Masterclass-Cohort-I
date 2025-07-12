enum Color {
    White(u8, u8, u8),
    Black(u8, u8, u8),
}

struct ShippingBox {
    dimensions: u32,
    weight: u32,
    color: Color,
}

impl ShippingBox {
    fn new_box(dimen: (u32, u32), wght: u32, color: Color) -> Self {
        let dimensions = dimen.0 * dimen.1;

        Self {
            dimensions,
            weight: wght,
            color,
        }
    }

    fn display_dim(&self) -> u32 {
        self.dimensions
    }

    fn display_wght(&self) -> u32 {
        self.weight
    }

    fn display_color(&self) -> &Color {
        &self.color
    }
}
fn main() {
    let my_box = ShippingBox::new_box((10, 20), 5, Color::White(255, 255, 255));

    println!("Dimensions: {}", my_box.display_dim());
    println!("Weight: {}", my_box.display_wght());

    match my_box.display_color() {
        Color::White(r, g, b) => println!("Color: White({}, {}, {})", r, g, b),
        Color::Black(r, g, b) => println!("Color: Black({}, {}, {})", r, g, b),
    }
}
