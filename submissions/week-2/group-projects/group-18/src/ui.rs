pub fn display_welcome() {
    println!("Welcome to the Customer Feedback Logger!");
    println!("This system helps you manage customer feedback for your business.");
    println!("Data is automatically saved after each operation and loaded on startup.");
}

pub fn display_menu() {
    println!("\n=== CUSTOMER FEEDBACK LOGGER ===");
    println!("1. Add Feedback");
    println!("2. View All Feedback");
    println!("3. Remove Feedback");
    println!("4. Edit Feedback");
    println!("5. Save Data");
    println!("6. Exit");
}

pub fn display_goodbye() {
    println!("Thank you for using Customer Feedback Logger. Goodbye!");
}
