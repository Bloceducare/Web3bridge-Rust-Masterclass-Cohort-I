// Enum representing possible box colors
#[derive(Debug)]
enum BoxColor {
    Red,
    Blue,
    Green,
}

// Struct representing a shipping box
struct BoxInfo {
    length: u64,
    width: u64,
    height: u64,
    weight: u64,
    color: BoxColor,
}

impl BoxInfo {
    // Associated function to create a new box
    fn create(length: u64, width: u64, height: u64, weight: u64, color: BoxColor) -> Self {
        Self {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    // Method to display box details
    fn show_details(&self) {
        println!("Box Dimensions: {} x {} x {}", self.length, self.width, self.height);
        println!("Weight: {} kg", self.weight);
        println!("Color: {:?}", self.color);
    }
}

fn main() {
    let package = BoxInfo::create(12, 8, 6, 15, BoxColor::Green);
    package.show_details();
}
