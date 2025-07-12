#[derive(Debug)]
enum Book {
    Fiction { author: String, price: f64 },
    Magazine { author: String, price: f64 },
    SciFi { price: f64 },
}

fn main() {
    let books = vec![
        Book::Fiction {
            author: String::from("Akande Gbolahan"),
            price: 15.99,
        },
        Book::Magazine {
            author: String::from("Gbolahan Akande"),
            price: 7.50,
        },
        Book::SciFi {
            price: 22.95,
        },
    ];

    for book in books {
        match book {
            Book::Fiction { author, price } => {
                println!("Fiction by {}, Price: £{}", author, price);
            },
            Book::Magazine { author, price } => {
                println!("Magazine by {}, Price: £{}", author, price);
            },
            Book::SciFi { price } => {
                println!("SciFi, Price: £{}", price);
            },
        }
    }
}
