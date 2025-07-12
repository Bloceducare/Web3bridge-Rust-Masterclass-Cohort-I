// Enum for Book types, each with associated data
enum Book {
    Fiction { author: String, price: f32 },
    Magazine { author: String, price: f32 },
    SciFi { price: f32 },
}

fn main() {
    // Create a vector of books
    let books = vec![
        Book::Fiction {
            author: "Chinua Achebe".to_string(),
            price: 15.0,
        },
        Book::Magazine {
            author: "Time Editorial".to_string(),
            price: 7.5,
        },
        Book::SciFi { price: 12.0 },
    ];

    // Print out each book's information
    for book in &books {
        match book {
            Book::Fiction { author, price } => {
                println!("Fiction Book by {} costs ${}", author, price);
            }
            Book::Magazine { author, price } => {
                println!("Magazine by {} costs ${}", author, price);
            }
            Book::SciFi { price } => {
                println!("SciFi Book costs ${}", price);
            }
        }
    }
}
