// Define an enum for different book types with associated data
#[derive(Debug)]
enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

fn main() {
    // Create instances of each book type
    let books = vec![
        Book::Fiction {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
            price: 12.99,
        },
        Book::Magazine {
            title: "National Geographic".to_string(),
            author: "Various Contributors".to_string(),
            price: 5.99,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 15.99,
        },
        Book::Fiction {
            title: "To Kill a Mockingbird".to_string(),
            author: "Harper Lee".to_string(),
            price: 10.99,
        },
        Book::Magazine {
            title: "Scientific American".to_string(),
            author: "Editorial Team".to_string(),
            price: 7.99,
        },
        Book::SciFi {
            title: "The Martian".to_string(),
            price: 13.99,
        },
    ];

    println!("Book Inventory:");
    println!("==================");

    // Iterate through the vector and use match to print book information
    for (index, book) in books.iter().enumerate() {
        print!("{}. ", index + 1);
        
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction: \"{}\" by {} - ${:.2}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine: \"{}\" by {} - ${:.2}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("Sci-Fi: \"{}\" - ${:.2}", title, price);
            }
        }
    }

    // Bonus: Calculate total inventory value
    let total_value: f64 = books.iter().map(|book| {
        match book {
            Book::Fiction { price, .. } => *price,
            Book::Magazine { price, .. } => *price,
            Book::SciFi { price, .. } => *price,
        }
    }).sum();

    println!("\nTotal inventory value: ${:.2}", total_value);
}
