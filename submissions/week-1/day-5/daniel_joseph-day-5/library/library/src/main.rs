enum Book {
    Fiction { author: String, price: f32 },
    Magazine { author: String, price: f32 },
    SciFi { price: f32 },
}

fn main() {
    // Create one of each book
    let book1 = Book::Fiction {
        author: String::from("Chinua Achebe"),
        price: 3500.0,
    };

    let book2 = Book::Magazine {
        author: String::from("Time Publishing"),
        price: 1500.0,
    };

    let book3 = Book::SciFi {
        price: 4200.0,
    };

    // Store them in a vector
    let books = vec![book1, book2, book3];

    // Iterate and print info using match
    for book in books {
        match book {
            Book::Fiction { author, price } => {
                println!("Fiction Book by {} - ₦{}", author, price);
            }
            Book::Magazine { author, price } => {
                println!("Magazine by {} - ₦{}", author, price);
            }
            Book::SciFi { price } => {
                println!("Sci-Fi Book - ₦{}", price);
            }
        }
    }
}
