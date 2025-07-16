#[derive(Debug, Clone)]
pub enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

fn main() {
    let books: Vec<Book> = vec![
        Book::Fiction {
            title: String::from("Things Fall Apart"),
            author: String::from("Chinua Achebe"),
            price: 10.00,
        },
        Book::Magazine {
            title: String::from("Genevieve Magazine"),
            author: String::from("Betty Irabor"),
            price: 5.99,
        },
        Book::SciFi {
            title: String::from("Lagoon"),
            price: 15.00,
        },
    ];

    for book in &books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction: {} by {} - ${:.2}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine: {} by {} - ${:.2}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("SciFi: {} - ${:.2}", title, price);
            }
        }
    }
}