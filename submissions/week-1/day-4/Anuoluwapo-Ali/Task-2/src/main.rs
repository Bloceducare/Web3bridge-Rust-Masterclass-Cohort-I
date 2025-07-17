fn main() {
    let small_box = ShippingBox::new(10.0, 8.0, 6.0, 2.5, BoxColor::Red);
    let medium_box = ShippingBox::new(20.0, 15.0, 12.0, 5.0, BoxColor::Blue);
    let large_box = ShippingBox::new(30.0, 25.0, 18.0, 8.2, BoxColor::Blue);
    
    small_box.print_characteristics();
    println!();
    
    medium_box.print_characteristics();
    println!();
    
    large_box.print_characteristics();
    println!();
    
   
    println!("Small box volume: {:.2} cubic units", small_box.calculate_volume());
    println!("Medium box is fragile: {}", medium_box.is_fragile());
    println!("Large box surface area: {:.2} square units", large_box.calculate_surface_area());
}


struct ShippingBox {
    length: f64,
    width: f64,
    height: f64,
    weight: f64,
    color: BoxColor,
}

enum BoxColor {
    Blue,
    Green,
    Red,
}

impl ShippingBox {
    fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }
    
    fn print_characteristics(&self) {
        println!("   Dimensions: {:.1} x {:.1} x {:.1}", self.length, self.width, self.height);
        println!("   Weight: {:.1} kg", self.weight);
        println!("   Color: {}", self.color_to_string());
    }
    
    fn color_to_string(&self) -> &str {
        match self.color {
            BoxColor::Blue => "Blue",
            BoxColor::Green => "Green",
            BoxColor::Red => "Red",
        }
    }

    fn calculate_volume(&self) -> f64 {
        self.length * self.width * self.height
    }

    fn is_fragile(&self) -> bool {
        self.weight > 7.0
    }
    
    fn calculate_surface_area(&self) -> f64 {
        2.0 * (self.length * self.width + self.width * self.height + self.height * self.length)
    }
}

impl BoxColor {
    fn description(&self) -> &str {
        match self {

            BoxColor::Blue => " branding",
            BoxColor::Green => " packaging",
            BoxColor::Red => " shipping",
        }
    }
}