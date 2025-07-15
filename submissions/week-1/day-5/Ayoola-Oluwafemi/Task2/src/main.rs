// Define the Book enum with associated data
enum Book {
    Fiction { title: String, author: String, price: f32 },
    Magazine { title: String, author: String, price: f32 },
    SciFi { title: String, price: f32 },
}

fn main() {
    // Create different types of books
    let book1 = Book::Fiction {
        title: "The Great Escape".to_string(),
        author: "John Smith".to_string(),
        price: 15.99,
    };

    let book2 = Book::Magazine {
        title: "Tech Monthly".to_string(),
        author: "Alice Johnson".to_string(),
        price: 6.50,
    };

    let book3 = Book::SciFi {
        title: "Galactic Wars".to_string(),
        price: 12.00,
    };

    // Store them in a vector
    let books = vec![book1, book2, book3];

    // Print book info using match
    for book in books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction Book: {}\nAuthor: {}\nPrice: ${:.2}\n", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine: {}\nAuthor: {}\nPrice: ${:.2}\n", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("Sci-Fi Book: {}\nPrice: ${:.2}\n", title, price);
            }
        }
    }
}
