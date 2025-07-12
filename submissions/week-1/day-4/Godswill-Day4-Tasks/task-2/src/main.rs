// Task 2: Shipping Box System with impl keyword

/// Represents different colors available for shipping boxes
#[derive(Debug, Clone, PartialEq)]
pub enum BoxColor {
    Brown,
    White,
    Black,
    Blue,
    Red,
    Green,
}

/// Represents a shipping box with dimensions, weight, and color
#[derive(Debug, Clone)]
pub struct ShippingBox {
    /// Length of the box in centimeters
    pub length: f64,
    /// Width of the box in centimeters
    pub width: f64,
    /// Height of the box in centimeters
    pub height: f64,
    /// Weight of the box in kilograms
    pub weight: f64,
    /// Color of the shipping box
    pub color: BoxColor,
}

impl ShippingBox {
    /// Creates a new shipping box with the specified characteristics
    /// 
    /// # Arguments
    /// * `length` - Length of the box in centimeters
    /// * `width` - Width of the box in centimeters
    /// * `height` - Height of the box in centimeters
    /// * `weight` - Weight of the box in kilograms
    /// * `color` - Color of the shipping box
    /// 
    /// # Returns
    /// A new ShippingBox instance
    pub fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    /// Prints all characteristics of the shipping box
    pub fn print_characteristics(&self) {
        println!("\n === SHIPPING BOX CHARACTERISTICS ===");
        println!(" Dimensions: {:.2} x {:.2} x {:.2} cm", self.length, self.width, self.height);
        println!("  Weight: {:.2} kg", self.weight);
        println!(" Color: {:?}", self.color);
        println!(" Volume: {:.2} cubic cm", self.calculate_volume());
        println!("=======================================\n");
    }

    /// Calculates the volume of the shipping box
    /// 
    /// # Returns
    /// The volume in cubic centimeters
    pub fn calculate_volume(&self) -> f64 {
        self.length * self.width * self.height
    }
}

fn main() {
    println!("TASK 2: SHIPPING BOX SYSTEM");
    println!("===============================");
    
    // Create shipping boxes using the new method
    let small_box = ShippingBox::new(20.0, 15.0, 10.0, 0.5, BoxColor::Brown);
    let medium_box = ShippingBox::new(40.0, 30.0, 20.0, 1.2, BoxColor::Blue);
    let large_box = ShippingBox::new(60.0, 45.0, 35.0, 2.8, BoxColor::Green);
    
    // Print characteristics of each box
    println!("Small Package:");
    small_box.print_characteristics();
    
    println!("Medium Package:");
    medium_box.print_characteristics();
    
    println!("Large Package:");
    large_box.print_characteristics();
    
    // Demonstrate additional functionality
    println!("Additional Box Analysis:");
    println!("Small box volume: {:.2} cm³", small_box.calculate_volume());
    println!("Medium box volume: {:.2} cm³", medium_box.calculate_volume());
    println!("Large box volume: {:.2} cm³", large_box.calculate_volume());
    
    let total_volume = small_box.calculate_volume() + medium_box.calculate_volume() + large_box.calculate_volume();
    println!("Total volume of all boxes: {:.2} cm³", total_volume);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for Task 2: Shipping Box System
    #[test]
    fn test_shipping_box_new() {
        let box_ = ShippingBox::new(10.0, 5.0, 2.0, 0.1, BoxColor::White);
        assert_eq!(box_.length, 10.0);
        assert_eq!(box_.width, 5.0);
        assert_eq!(box_.height, 2.0);
        assert_eq!(box_.weight, 0.1);
        assert_eq!(box_.color, BoxColor::White);
    }

    #[test]
    fn test_calculate_volume() {
        let box_ = ShippingBox::new(10.0, 5.0, 2.0, 0.1, BoxColor::White);
        assert_eq!(box_.calculate_volume(), 100.0);
    }
}