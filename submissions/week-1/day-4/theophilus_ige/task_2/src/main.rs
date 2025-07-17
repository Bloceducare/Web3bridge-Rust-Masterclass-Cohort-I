struct ShippingBox {
    length: i32,
    width: i32,
    height: i32,
    weight: i32,
    color: Color,
}

enum Color {
    Red,
    Blue,
    Green,
}

impl ShippingBox {
    fn new(length: i32, width: i32, height: i32, weight: i32, color: Color) -> ShippingBox {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn shoppingbox_characteristics(&self) {
        println!("Dimensions: {} x {} x {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);

        match self.color {
            Color::Red => println!("Color: Red"),
            Color::Blue => println!("Color: Blue"),
            Color::Green => println!("Color: Green"),
        }
    }
}

fn main() {
    let my_box = ShippingBox::new(10, 2, 3, 1, Color::Blue);

    my_box.shoppingbox_characteristics();
}
