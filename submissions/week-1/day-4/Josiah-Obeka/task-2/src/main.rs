#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(length: f32, width: f32, height: f32, weight: f32, color: Color) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn display_characteristics(&self) {
        println!(
            "Box Dimensions: {} . {} . {}",
            self.length, self.width, self.height
        );
        println!("Weight: {} kg", self.weight);
        println!(
            "Color: {}",
            match self.color {
                Color::Red => "Red",
                Color::Blue => "Blue",
                Color::Green => "Green",
            }
        );
    }
}

fn main() {
    let my_box = ShippingBox::new(10.0, 5.0, 8.0, 2.5, Color::Blue);
    my_box.display_characteristics();
}
