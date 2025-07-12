#![allow(unused)]

enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

impl Book {
    fn print_info(&self) {
        match self {
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

fn main() {
    let books = vec![
        Book::Fiction {
            title: String::from("The Great Gatsby"),
            author: String::from("F. Scott Fitzgerald"),
            price: 9.99,
        },
        Book::Magazine {
            title: String::from("National Geographic"),
            author: String::from("Various"),
            price: 5.99,
        },
        Book::SciFi {
            title: String::from("Dune"),
            price: 12.99,
        },
    ];

    println!("Book Catalog:");
    for book in &books {
        book.print_info();
    }
}