
#[derive(Debug)]
#[allow(dead_code)]
enum BoxColor {
    Brown,
    White,
    Blue,
    Red,
    Green,
    Black,
}


struct ShippingBox {
    length: f64,
    width: f64,
    height: f64,
    weight: f64,
    color: BoxColor,
}


impl ShippingBox {
    fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> ShippingBox {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    
    fn print_characteristics(&self) {
        println!("=== Shipping Box Characteristics ===");
        println!("Dimensions:");
        println!("  Length: {:.2} cm", self.length);
        println!("  Width:  {:.2} cm", self.width);
        println!("  Height: {:.2} cm", self.height);
        println!("Volume: {:.2} cubic cm", self.length * self.width * self.height);
        println!("Weight: {:.2} kg", self.weight);
        
        match &self.color {
            BoxColor::Brown => println!("Color: Brown"),
            BoxColor::White => println!("Color: White"),
            BoxColor::Blue => println!("Color: Blue"),
            BoxColor::Red => println!("Color: Red"),
            BoxColor::Green => println!("Color: Green"),
            BoxColor::Black => println!("Color: Black"),
        }
        println!("================================");
    }


    fn calculate_volume(&self) -> f64 {
        self.length * self.width * self.height
    }


    fn is_heavy(&self) -> bool {
        self.weight > 10.0
    }
}

fn main() {

    let small_box = ShippingBox::new(20.0, 15.0, 10.0, 2.5, BoxColor::Brown);
    let medium_box = ShippingBox::new(30.0, 25.0, 20.0, 8.0, BoxColor::White);
    let large_box = ShippingBox::new(50.0, 40.0, 30.0, 15.0, BoxColor::Blue);
    let red_box = ShippingBox::new(25.0, 20.0, 15.0, 5.0, BoxColor::Red);


    small_box.print_characteristics();
    medium_box.print_characteristics();
    large_box.print_characteristics();
    red_box.print_characteristics();


    
    println!("\n=== Additional Information ===");
    println!("Small box volume: {:.2} cubic cm", small_box.calculate_volume());
    println!("Medium box volume: {:.2} cubic cm", medium_box.calculate_volume());
    println!("Large box volume: {:.2} cubic cm", large_box.calculate_volume());
    
    println!("\nHeavy boxes (over 10kg):");
    println!("Small box is heavy: {}", small_box.is_heavy());
    println!("Medium box is heavy: {}", medium_box.is_heavy());
    println!("Large box is heavy: {}", large_box.is_heavy());
}
