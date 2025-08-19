enum Book {
    Fiction {
        title: String,
        author: String,
        price: f32,
    },
    Magazine {
        title: String,
        author: String,
        price: f32,
    },
    SciFi {
        title: String,
        author: String,
        volume: String,
        price: f32,
    },
}

fn main() {
    let books = vec![
        Book::Fiction {
            title: String::from("Things fall apart"),
            author: String::from("Sir. Chief Chinhua Achiebe"),
            price: 15.99,
        },
        Book::Magazine {
            title: String::from("Traditional Home"),
            author: String::from("Ferguson Showrooms"),
            price: 9.49,
        },
        Book::SciFi {
            title: String::from("Superman: For the man who has Everything"),
            author: String::from("DC Comics"),
            volume: String::from("One"),
            price: 12.75,
        },
    ];

    for book in &books {
        match book {
            Book::Fiction {
                title,
                author,
                price,
            } => {
                println!("Fiction: \"{}\" by {}, ${}", title, author, price);
            }
            Book::Magazine {
                title,
                author,
                price,
            } => {
                println!("Magazine: \"{}\" by {}, ${}", title, author, price);
            }
            Book::SciFi { title, price, .. } => {
                println!("SciFi: \"{}\", ${}", title, price);
            }
        }
    }
}
