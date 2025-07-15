enum Book {
    Fiction { title: String, author: String, price: f32 },
    Magazine { title: String, author: String, price: f32 },
    SciFi { title: String, price: f32 },
}

fn main() {
    let books = vec![
        Book::Fiction {
            title: "Things Fall Apart".to_string(),
            author: "Chinua Achebe".to_string(),
            price: 10.99,
        },
        Book::Magazine {
            title: "The Nations".to_string(),
            author: "Various".to_string(),
            price: 5.50,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 12.00,
        },
    ];

    for book in books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction - Title: {}, Author: {}, Price: ${}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine - Title: {}, Author: {}, Price: ${}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("SciFi - Title: {}, Price: ${}", title, price);
            }
        }
    }
}
