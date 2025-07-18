// Task 1: Library Item Management System
// Task 2: Shipping Box System with impl keyword

/// Represents different types of items available in the library
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Book,
    Magazine,
    Fiction,
    NonFiction,
    Reference,
}

/// Represents a library item with quantity, ID, and type
#[derive(Debug, Clone)]
pub struct LibraryItem {
    /// The quantity of this item available in the library
    pub quantity: i32,
    /// Unique identifier for this item
    pub id: i32,
    /// The type/category of this library item
    pub item_type: ItemType,
}

/// Displays the quantity of a library item
/// 
/// # Arguments
/// * `item` - A reference to the LibraryItem to display quantity for
pub fn display_quantity(item: &LibraryItem) {
    println!("📚 Item Quantity: {}", item.quantity);
}

/// Displays the ID of a library item
/// 
/// # Arguments
/// * `item` - A reference to the LibraryItem to display ID for
pub fn display_id(item: &LibraryItem) {
    println!("🔢 Item ID: {}", item.id);
}

/// Displays the type of a library item
/// 
/// # Arguments
/// * `item` - A reference to the LibraryItem to display type for
pub fn display_item_type(item: &LibraryItem) {
    let type_emoji = match item.item_type {
        ItemType::Book => "📖",
        ItemType::Magazine => "📰",
        ItemType::Fiction => "🎭",
        ItemType::NonFiction => "📚",
        ItemType::Reference => "📋",
    };
    println!("{} Item Type: {:?}", type_emoji, item.item_type);
}

// Task 2: Shipping Box System

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
        println!("\n📦 === SHIPPING BOX CHARACTERISTICS ===");
        println!("📏 Dimensions: {:.2} x {:.2} x {:.2} cm", self.length, self.width, self.height);
        println!("⚖️  Weight: {:.2} kg", self.weight);
        println!("🎨 Color: {:?}", self.color);
        println!("📊 Volume: {:.2} cubic cm", self.calculate_volume());
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
    println!("🎯 === WEEK 1 DAY 4 TASKS - GODSWILL ===\n");
    
    // Task 1: Library Item Management System
    println!("📚 TASK 1: LIBRARY ITEM MANAGEMENT SYSTEM");
    println!("==========================================");
    
    // Create some library items
    let book = LibraryItem {
        quantity: 15,
        id: 1001,
        item_type: ItemType::Book,
    };
    
    let magazine = LibraryItem {
        quantity: 25,
        id: 2001,
        item_type: ItemType::Magazine,
    };
    
    let fiction_book = LibraryItem {
        quantity: 8,
        id: 3001,
        item_type: ItemType::Fiction,
    };
    
    // Display information for each library item
    println!("\n📖 Processing Library Item #1:");
    display_quantity(&book);
    display_id(&book);
    display_item_type(&book);
    
    println!("\n📰 Processing Library Item #2:");
    display_quantity(&magazine);
    display_id(&magazine);
    display_item_type(&magazine);
    
    println!("\n🎭 Processing Library Item #3:");
    display_quantity(&fiction_book);
    display_id(&fiction_book);
    display_item_type(&fiction_book);
    
    // Task 2: Shipping Box System with impl
    println!("\n\n📦 TASK 2: SHIPPING BOX SYSTEM");
    println!("===============================");
    
    // Create shipping boxes using the new method
    let small_box = ShippingBox::new(20.0, 15.0, 10.0, 0.5, BoxColor::Brown);
    let medium_box = ShippingBox::new(40.0, 30.0, 20.0, 1.2, BoxColor::Blue);
    let large_box = ShippingBox::new(60.0, 45.0, 35.0, 2.8, BoxColor::Green);
    
    // Print characteristics of each box
    println!("📦 Small Package:");
    small_box.print_characteristics();
    
    println!("📦 Medium Package:");
    medium_box.print_characteristics();
    
    println!("📦 Large Package:");
    large_box.print_characteristics();
    
    // Demonstrate additional functionality
    println!("🔍 Additional Box Analysis:");
    println!("Small box volume: {:.2} cm³", small_box.calculate_volume());
    println!("Medium box volume: {:.2} cm³", medium_box.calculate_volume());
    println!("Large box volume: {:.2} cm³", large_box.calculate_volume());
    
    let total_volume = small_box.calculate_volume() + medium_box.calculate_volume() + large_box.calculate_volume();
    println!("📊 Total volume of all boxes: {:.2} cm³", total_volume);
    
    println!("\n✅ All tasks completed successfully! 🎉");
} 