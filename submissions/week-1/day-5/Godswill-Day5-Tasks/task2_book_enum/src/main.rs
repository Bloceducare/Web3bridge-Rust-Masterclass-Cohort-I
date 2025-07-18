// Task 2: Advanced Match Using Enums
// This program demonstrates enums with associated data and pattern matching

// Define our Book enum with associated data
// Each variant can hold different types and amounts of data
#[derive(Debug, Clone)] // Debug allows us to print the enum, Clone allows copying
enum Book {
    // Fiction books have title, author, and price
    Fiction { 
        title: String, 
        author: String, 
        price: f32 
    },
    // Magazine books also have title, author, and price
    Magazine { 
        title: String, 
        author: String, 
        price: f32 
    },
    // SciFi books only have title and price (no author field)
    SciFi { 
        title: String, 
        price: f32 
    },
}

// Implementation block for our Book enum
impl Book {
    // Method to get the price of any book type
    // This demonstrates how to match on enum variants
    fn get_price(&self) -> f32 {
        match self {
            // For each variant, we destructure and return the price
            Book::Fiction { price, .. } => *price,      // .. means "ignore other fields"
            Book::Magazine { price, .. } => *price,
            Book::SciFi { price, .. } => *price,
        }
    }
    
    // Method to get the title of any book type
    fn get_title(&self) -> &str {
        match self {
            Book::Fiction { title, .. } => title,
            Book::Magazine { title, .. } => title,
            Book::SciFi { title, .. } => title,
        }
    }
    
    // Method to get the book type as a string
    fn get_type(&self) -> &str {
        match self {
            Book::Fiction { .. } => "Fiction",
            Book::Magazine { .. } => "Magazine",
            Book::SciFi { .. } => "SciFi",
        }
    }
    
    // Method to check if the book has an author
    fn has_author(&self) -> bool {
        match self {
            Book::Fiction { .. } => true,
            Book::Magazine { .. } => true,
            Book::SciFi { .. } => false,
        }
    }
}

// Function to create a collection of books
// This demonstrates creating different enum variants
fn create_book_collection() -> Vec<Book> {
    println!("📚 Creating a diverse book collection...\n");
    
    vec![
        // Creating Fiction books
        Book::Fiction {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            price: 12.99,
        },
        Book::Fiction {
            title: "1984".to_string(),
            author: "George Orwell".to_string(),
            price: 13.99,
        },
        Book::Fiction {
            title: "Pride and Prejudice".to_string(),
            author: "Jane Austen".to_string(),
            price: 11.50,
        },
        
        // Creating Magazine books
        Book::Magazine {
            title: "National Geographic".to_string(),
            author: "Various Contributors".to_string(),
            price: 6.99,
        },
        Book::Magazine {
            title: "Scientific American".to_string(),
            author: "Science Editorial Team".to_string(),
            price: 8.50,
        },
        
        // Creating SciFi books (no author field)
        Book::SciFi {
            title: "Dune".to_string(),
            price: 16.99,
        },
        Book::SciFi {
            title: "The Martian".to_string(),
            price: 14.50,
        },
        Book::SciFi {
            title: "Neuromancer".to_string(),
            price: 13.25,
        },
    ]
}

// Function to display a single book using match
// This is the core requirement - using match to print book info
fn display_book(book: &Book, index: usize) {
    print!("{}. ", index + 1);
    
    // This is the main match expression required by the task
    match book {
        // For Fiction books, we destructure all fields and display them
        Book::Fiction { title, author, price } => {
            println!("📖 Fiction: \"{}\" by {} - ${:.2}", title, author, price);
        }
        
        // For Magazine books, we also have author information
        Book::Magazine { title, author, price } => {
            println!("📰 Magazine: \"{}\" by {} - ${:.2}", title, author, price);
        }
        
        // For SciFi books, we only have title and price (no author)
        Book::SciFi { title, price } => {
            println!("🚀 SciFi: \"{}\" - ${:.2}", title, price);
        }
    }
}

// Function to display all books in the collection
fn display_book_collection(books: &Vec<Book>) {
    println!("📋 Book Collection Inventory:");
    println!("{}", "=".repeat(70));
    
    if books.is_empty() {
        println!("📭 No books in the collection.");
    } else {
        // Iterate through the vector and display each book
        for (index, book) in books.iter().enumerate() {
            display_book(book, index);
        }
    }
    
    println!("{}", "=".repeat(70));
}

// Function to demonstrate advanced matching patterns
fn demonstrate_advanced_matching(books: &Vec<Book>) {
    println!("\n🔍 Advanced Matching Demonstrations:");
    println!("{}", "=".repeat(50));
    
    // Example 1: Count books by type
    let mut fiction_count = 0;
    let mut magazine_count = 0;
    let mut scifi_count = 0;
    
    for book in books {
        match book {
            Book::Fiction { .. } => fiction_count += 1,
            Book::Magazine { .. } => magazine_count += 1,
            Book::SciFi { .. } => scifi_count += 1,
        }
    }
    
    println!(" Book Count by Type:");
    println!("   Fiction: {}", fiction_count);
    println!("   Magazine: {}", magazine_count);
    println!("   SciFi: {}", scifi_count);
    
    // Example 2: Find expensive books (price > $15)
    println!("\n Expensive Books (> $15.00):");
    for book in books {
        match book {
            Book::Fiction { title, price, .. } if *price > 15.0 => {
                println!("    Fiction: \"{}\" - ${:.2}", title, price);
            }
            Book::Magazine { title, price, .. } if *price > 15.0 => {
                println!("    Magazine: \"{}\" - ${:.2}", title, price);
            }
            Book::SciFi { title, price } if *price > 15.0 => {
                println!("    SciFi: \"{}\" - ${:.2}", title, price);
            }
            _ => {} // Ignore books that don't match the price condition
        }
    }
    
    // Example 3: Books with specific authors
    println!("\n Books by George Orwell:");
    for book in books {
        match book {
            Book::Fiction { title, author, price } if author == "George Orwell" => {
                println!("    \"{}\" - ${:.2}", title, price);
            }
            Book::Magazine { title, author, price } if author == "George Orwell" => {
                println!("    \"{}\" - ${:.2}", title, price);
            }
            _ => {} // Ignore books not by George Orwell
        }
    }
}

// Function to calculate statistics using match
fn calculate_statistics(books: &Vec<Book>) {
    println!("\n Collection Statistics:");
    println!("{}", "=".repeat(40));
    
    let mut total_value = 0.0;
    let mut cheapest_price = f32::MAX;
    let mut most_expensive_price = 0.0;
    let mut cheapest_book = String::new();
    let mut most_expensive_book = String::new();
    
    for book in books {
        let price = book.get_price();
        let title = book.get_title();
        
        total_value += price;
        
        if price < cheapest_price {
            cheapest_price = price;
            cheapest_book = title.to_string();
        }
        
        if price > most_expensive_price {
            most_expensive_price = price;
            most_expensive_book = title.to_string();
        }
    }
    
    println!(" Total books: {}", books.len());
    println!(" Total collection value: ${:.2}", total_value);
    println!(" Average book price: ${:.2}", total_value / books.len() as f32);
    println!(" Cheapest book: \"{}\" - ${:.2}", cheapest_book, cheapest_price);
    println!(" Most expensive book: \"{}\" - ${:.2}", most_expensive_book, most_expensive_price);
}

// Main function - demonstrates all the required functionality
fn main() {
    println!(" Welcome to the Advanced Book Enum Matcher!");
    println!("This program demonstrates enums with associated data and pattern matching.\n");
    
    // Create our book collection as required by the task
    let books = create_book_collection();
    
    // Display the collection using match expressions (main requirement)
    display_book_collection(&books);
    
    // Demonstrate advanced matching patterns
    demonstrate_advanced_matching(&books);
    
    // Calculate and display statistics
    calculate_statistics(&books);
    
    println!("\n Key Concepts Demonstrated:");
    println!("{}", "=".repeat(50));
    println!(" Enum with associated data for each variant");
    println!(" Vector containing different enum variants");
    println!(" Match expressions to handle each variant");
    println!(" Pattern matching with destructuring");
    println!(" Conditional matching with guards (if conditions)");
    println!(" Methods on enums");
    println!(" Iteration with match expressions");
    
    println!("\n Book Enum Matcher demonstration completed!");
    println!("All requirements have been fulfilled using Rust best practices.");
}
