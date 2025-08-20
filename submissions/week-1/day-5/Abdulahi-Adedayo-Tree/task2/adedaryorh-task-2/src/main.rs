#[derive(Debug)]
enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

impl Book {
    fn get_title(&self) -> &str {
        match self {
            Book::Fiction { title, .. } => title,
            Book::Magazine { title, .. } => title,
            Book::SciFi { title, .. } => title,
        }
    }

    fn get_price(&self) -> f64 {
        match self {
            Book::Fiction { price, .. } => *price,
            Book::Magazine { price, .. } => *price,
            Book::SciFi { price, .. } => *price,
        }
    }
}

fn print_book_info(book: &Book) {
    match book {
        Book::Fiction { title, author, price } => {
            println!("📚 Fiction Book:");
            println!("   Title: {}", title);
            println!("   Author: {}", author);
            println!("   Price: ${:.2}", price);
        }
        Book::Magazine { title, author, price } => {
            println!("📰 Magazine:");
            println!("   Title: {}", title);
            println!("   Author: {}", author);
            println!("   Price: ${:.2}", price);
        }
        Book::SciFi { title, price } => {
            println!("🚀 Science Fiction Book:");
            println!("   Title: {}", title);
            println!("   Price: ${:.2}", price);
        }
    }
    println!("─────────────────────────────────────");
}

fn main() {
    println!("📖 Book Collection Manager");
    println!("===========================\n");

    // Create a vector of different book types
    let books: Vec<Book> = vec![
        Book::Fiction {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            price: 14.99,
        },
        Book::Magazine {
            title: "National Geographic".to_string(),
            author: "Various Contributors".to_string(),
            price: 6.99,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 18.50,
        },
        Book::Fiction {
            title: "Pride and Prejudice".to_string(),
            author: "Jane Austen".to_string(),
            price: 12.99,
        },
        Book::Magazine {
            title: "Scientific American".to_string(),
            author: "Editorial Team".to_string(),
            price: 7.99,
        },
        Book::SciFi {
            title: "The Hitchhiker's Guide to the Galaxy".to_string(),
            price: 15.99,
        },
        Book::Fiction {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
            price: 13.50,
        },
        Book::SciFi {
            title: "Foundation".to_string(),
            price: 16.99,
        },
    ];

    println!("📋 Book Inventory ({} books):\n", books.len());

    // Iterate through the vector and use match to print book info
    for (index, book) in books.iter().enumerate() {
        println!("Book #{}", index + 1);
        print_book_info(book);
        println!();
    }

    // Additional statistics using match
    println!("📊 Collection Statistics:");
    println!("========================");
    
    let mut fiction_count = 0;
    let mut magazine_count = 0;
    let mut scifi_count = 0;
    let mut total_value = 0.0;

    for book in &books {
        match book {
            Book::Fiction { price, .. } => {
                fiction_count += 1;
                total_value += price;
            }
            Book::Magazine { price, .. } => {
                magazine_count += 1;
                total_value += price;
            }
            Book::SciFi { price, .. } => {
                scifi_count += 1;
                total_value += price;
            }
        }
    }

    println!("📚 Fiction books: {}", fiction_count);
    println!("📰 Magazines: {}", magazine_count);
    println!("🚀 Science Fiction books: {}", scifi_count);
    println!("💰 Total collection value: ${:.2}", total_value);
    
    // Find the most expensive book
    let most_expensive = books.iter()
        .max_by(|a, b| a.get_price().partial_cmp(&b.get_price()).unwrap());
    
    if let Some(book) = most_expensive {
        println!("\n💎 Most expensive book: '{}' - ${:.2}", 
                 book.get_title(), book.get_price());
    }

    println!("\n🎉 Book collection display completed!");
}