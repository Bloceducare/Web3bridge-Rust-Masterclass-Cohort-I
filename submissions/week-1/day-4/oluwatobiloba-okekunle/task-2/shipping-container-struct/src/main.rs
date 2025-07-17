enum BoxColor {
    Brown,
    White,
    Black,
    Blue,
}

struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor,
}


impl ShippingBox {
    // Constructor method to create a new box
    fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> ShippingBox {
        ShippingBox {
            dimensions: Dimensions {
                length,
                width,
                height,
            },
            weight,
            color,
        }
    }

    // Method to print box characteristics
    fn display_characteristics(&self) {
        println!("Box Characteristics:");
        println!("-------------------");
        println!("Dimensions:");
        println!("  Length: {} units", self.dimensions.length);
        println!("  Width: {} units", self.dimensions.width);
        println!("  Height: {} units", self.dimensions.height);
        println!("Weight: {} kg", self.weight);
        
        // Match expression to convert enum to string
        let color_str = match self.color {
            BoxColor::Brown => "Brown",
            BoxColor::White => "White",
            BoxColor::Black => "Black",
            BoxColor::Blue => "Blue",
        };
        println!("Color: {}", color_str);
    }
}

fn main() {
    // Create a new shipping box using the constructor
    let my_box = ShippingBox::new(
        20.0,
        15.0,
        10.0,
        5.5,
        BoxColor::Brown,
    );

    my_box.display_characteristics();

    let my_box = ShippingBox::new(
        20.0,
        15.0,
        10.0,
        5.5,
        BoxColor::Black,
    );

    my_box.display_characteristics();

    let my_box = ShippingBox::new(
        10.0,
        15.0,
        20.0,
        4.5,
        BoxColor::Blue,
    );

    my_box.display_characteristics();

    let my_box = ShippingBox::new(
        10.0,
        15.0,
        20.0,
        4.5,
        BoxColor::White,
    );

    my_box.display_characteristics();

    let my_box = ShippingBox::new(
        10.0,
        15.0,
        20.0,
        4.5,
        BoxColor::Black,
    );

    my_box.display_characteristics();
}