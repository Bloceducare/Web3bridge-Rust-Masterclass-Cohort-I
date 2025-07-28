#[derive(Debug)]
enum Book {
    Fiction {
        title: String,
        author: String,
        price: f64,
    },
    Magazine {
        title: String,
        author: String,
        price: f64,
    },
    SciFi {
        title: String,
        price: f64,
    },
}

fn main() {
    let books = vec![
        Book::Fiction {
            title: String::from("The Great Gatsby"),
            author: String::from("F. Scott Fitzgerald"),
            price: 15.99,
        },
        Book::Magazine {
            title: String::from("National Geographic"),
            author: String::from("Various Authors"),
            price: 9.99,
        },
        Book::SciFi {
            title: String::from("Dune"),
            price: 12.99,
        },
    ];

    for book in books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction Book: {} by {} - ${:.2}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine: {} by {} - ${:.2}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("Science Fiction Book: {} - ${:.2}", title, price);
            }
        }
    }
}