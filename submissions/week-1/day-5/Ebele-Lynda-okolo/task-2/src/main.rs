enum Book {
    Fiction { title: String, author: String, price: f32 },
    Magazine { title: String, issue: String, price: f32 },
    SciFi { title: String, price: f32 },
}

fn main() {
    let my_books = vec![
        Book::Fiction {
            title: "Dream Count".to_string(),
            author: "Chimamanda Ngozi Adichie".to_string(),
            price: 15.99,
        },
        Book::Magazine {
            title: "National Geographic".to_string(),
            issue: "January 2025".to_string(),
            price: 6.99,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 12.50,
        },
    ];

    for book in my_books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("{} by {} - ${}", title, author, price);
            }
            Book::Magazine { title, issue, price } => {
                println!("{} {} - ${}", title, issue, price);
            }
            Book::SciFi { title, price } => {
                println!("{} - ${}", title, price);
            }
        }
    }
}