enum BookType {
    Fiction { author: String, price: f64 },
    SciFi { price: f64 },
    Magazine { author: String, price: f64 },
}

fn main() {
    let books = vec![
        BookType::Fiction {
            author: "J.K. Rowling".to_string(),
            price: 29.99,
        },
        BookType::SciFi { price: 24.99 },
        BookType::Magazine {
            author: "National Geographic Team".to_string(),
            price: 12.99,
        },
    ];

    for (index, book) in books.iter().enumerate() {
        print!("Book {}: ", index + 1);

        match book {
            BookType::Fiction { author, price } => {
                println!("Fiction - Author: {}, Price: ${:.2}", author, price);
            }
            BookType::SciFi { price } => {
                println!("Science Fiction - Price: ${:.2}", price);
            }
            BookType::Magazine { author, price } => {
                println!("Magazine - Author: {}, Price: ${:.2}", author, price);
            }
        }
    }
}
