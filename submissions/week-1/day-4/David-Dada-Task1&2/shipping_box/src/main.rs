#[derive(Copy, Clone, Debug)]
enum Color {
    Red,
    Green,
    Blue
}

struct ShippingBox {
    dimension: (u32, u32),
    weight: u32,
    color: Color
}

impl ShippingBox {
    fn new(dimension: (u32, u32), weight: u32, color: Color) -> Self {
        Self {
            dimension,
            weight,
            color 
        }
    }

    fn get_dimension(&self) -> (u32, u32) {
        self.dimension
    }

    fn get_weight(&self) -> u32 {
        self.weight
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

fn main() {
    println!("Creating your box: ");

    let shipping_box = ShippingBox::new((66, 33), 69, Color::Green); 

    println!("Box dimension: {:?}", shipping_box.get_dimension());
    println!("Box weight: {}", shipping_box.get_weight());
    println!("Box color: {:?}", shipping_box.get_color());
}
