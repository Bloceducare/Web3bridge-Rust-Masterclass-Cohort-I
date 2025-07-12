// src/main.rs

#[derive(Debug)]
enum Book {
    Fiction { title: String, author: String, price: f32 },
    Magazine { title: String, author: String, price: f32 },
    SciFi { title: String, price: f32 },
}

fn main() {
    let books = vec![
        Book::Fiction {
            title: "The Alchemist".to_string(),
            author: "Paulo Coelho".to_string(),
            price: 12.99,
        },
        Book::Magazine {
            title: "National Geographic".to_string(),
            author: "Various".to_string(),
            price: 5.99,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 9.99,
        },
    ];

    println!("\n Book List:");

    for book in &books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction - '{}' by {}, Price: ${}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine - '{}' by {}, Price: ${}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("SciFi - '{}' | Price: ${}", title, price);
            }
        }
    }
}
