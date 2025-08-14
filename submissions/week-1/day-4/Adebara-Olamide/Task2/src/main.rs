enum boxColor {
    Red,
    Blue,
    Green,
    Yellow,
}
struct ShippingBox {
    dimension: (f64, f64, f64),
    width: f64,
    box_color: boxColor,
}

impl ShippingBox {
    fn new(dimension: f64, width: f64, box_color: boxColor) -> Self {
        Self {
            dimension: (dimension, dimension, dimension),
            width,
            box_color,
        }
    }

    fn display_shipping_box(&self) {
        println!("Dimension: {}, {}, {}", self.dimension.0, self.dimension.1, self.dimension.2);
        println!("Width: {}", self.width);
        match self.box_color {
            boxColor::Red => println!("Color: Red"),
            boxColor::Blue => println!("Color: Blue"),
            boxColor::Green => println!("Color: Green"),
            boxColor::Yellow => println!("Color: Yellow"),
        }
    }
}
fn main() {
    let box1 = ShippingBox::new((10.0, 10.0, 10.0), 5.0, boxColor::Red);
    let box2 = ShippingBox::new((15.0, 11.0, 10.0), 7.5, boxColor::Blue);
    let box3 = ShippingBox::new((10.0, 30.0, 10.0), 10.0, boxColor::Green);

    box1.display_shipping_box();
    box2.display_shipping_box();
    box3.display_shipping_box();
}
