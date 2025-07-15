

enum BoxColor {
    Red,
    Green,
    Blue,
    Yellow,
}

struct ShippingBox {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    color: BoxColor,
}


impl ShippingBox {
    fn new(width: f64, height: f64, depth: f64, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            width,
            height,
            depth,
            weight,
            color,
        }
    }


    fn print_dimensions(&self) {
        println!("\nShippingBox Dimensions: {} x {} x {}", self.width, self.height, self.depth);
        println!("Weight: {} kg", self.weight);
        match self.color {
            BoxColor::Red => println!("Color: Red"),
            BoxColor::Green => println!("Color: Green"),
            BoxColor::Blue => println!("Color: Blue"),
            BoxColor::Yellow => println!("Color: Yellow"),
        }
    }
}


fn main() {
    println!("Temitope - Adekeye!\n");
    let box1 = ShippingBox::new(30.0, 20.0, 15.0, 2.5, BoxColor::Blue);
    box1.print_dimensions();
    let box2 = ShippingBox::new(25.0, 15.0, 10.0, 1.5, BoxColor::Red);
    box2.print_dimensions();
    let box3 = ShippingBox::new(40.0, 30.0, 20.0, 3.0, BoxColor::Green);
    box3.print_dimensions();
    let box4 = ShippingBox::new(35.0, 25.0, 18.0, 2.0, BoxColor::Yellow);
    box4.print_dimensions();
    let box5 = ShippingBox::new(50.0, 40.0, 30.0, 5.0, BoxColor::Blue);
    box5.print_dimensions();
}
