enum Book {
    Fiction { author: String, price: f64 },
    Magazine { author: String, price: f64 },
    SciFi { price: f64 },
}

fn main() {
    let books = vec![
        Book::Fiction {
            author: "Jane Austen".to_string(),
            price: 12.99,
        },
        Book::Magazine {
            author: "Scientific American Staff".to_string(),
            price: 4.99,
        },
        Book::SciFi { price: 15.99 },
    ];

    for book in &books {
        match book {
            Book::Fiction { author, price } => {
                println!("Fiction book by {} - Naira {:.2}", author, price);
            }
            Book::Magazine { author, price } => {
                println!("Magazine by {} - Naira{:.2}", author, price);
            }
            Book::SciFi { price } => {
                println!("SciFi book - Naira {:.2}", price);
            }
        }
    }
}