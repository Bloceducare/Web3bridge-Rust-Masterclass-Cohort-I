// Define an enum for different book types with associated data
enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

fn main() {
    println!("📚 Book Collection - Advanced Match with Enums\n");
    
    // Create a vector of books with different types
    let books = vec![
        Book::Fiction {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
            price: 12.99,
        },
        Book::Magazine {
            title: "National Geographic".to_string(),
            author: "Various Authors".to_string(),
            price: 8.50,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 15.99,
        },
        Book::Fiction {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            price: 11.99,
        },
        Book::Magazine {
            title: "Time Magazine".to_string(),
            author: "Editorial Team".to_string(),
            price: 6.99,
        },
        Book::SciFi {
            title: "1984".to_string(),
            price: 13.50,
        },
    ];

    println!("=== BOOK CATALOG ===");
    println!("{:-<60}", "");
    
    // Use match expression while iterating the vector to print book info
    for (index, book) in books.iter().enumerate() {
        print!("{}. ", index + 1);
        
        match book {
            Book::Fiction { title, author, price } => {
                println!("📖 Fiction: '{}' by {} - ${:.2}", title, author, price);
            },
            Book::Magazine { title, author, price } => {
                println!("📰 Magazine: '{}' by {} - ${:.2}", title, author, price);
            },
            Book::SciFi { title, price } => {
                println!("🚀 Sci-Fi: '{}' - ${:.2}", title, price);
            },
        }
    }
    
    println!("{:-<60}", "");
    
    // Calculate total value of the collection
    let total_value: f64 = books.iter().map(|book| {
        match book {
            Book::Fiction { price, .. } => *price,
            Book::Magazine { price, .. } => *price,
            Book::SciFi { price, .. } => *price,
        }
    }).sum();
    
    println!("💰 Total Collection Value: ${:.2}", total_value);
    println!("📊 Number of Books: {}", books.len());
    
    // Count books by type
    let fiction_count = books.iter().filter(|book| {
        matches!(book, Book::Fiction { .. })
    }).count();
    
    let magazine_count = books.iter().filter(|book| {
        matches!(book, Book::Magazine { .. })
    }).count();
    
    let scifi_count = books.iter().filter(|book| {
        matches!(book, Book::SciFi { .. })
    }).count();
    
    println!("\n=== COLLECTION STATISTICS ===");
    println!("📖 Fiction Books: {}", fiction_count);
    println!("📰 Magazines: {}", magazine_count);
    println!("🚀 Sci-Fi Books: {}", scifi_count);
    
    println!("\n🎉 Book catalog display complete!");
}
