enum Book {
    Fiction { title: String, author: String, price: f32 },
    Magazine { title: String, author: String, price: f32 },
    SciFi { title: String, price: f32 },
}

fn main() {
    use Book::*;

    let books = vec![
        Fiction {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik"),
            price: 25.99,
        },
        Magazine {
            title: String::from("Rustacean Weekly"),
            author: String::from("Rust Community"),
            price: 5.00,
        },
        SciFi {
            title: String::from("Dune"),
            price: 15.50,
        },
    ];

    println!(" Book List:");
    for book in &books {
        match book {
            Fiction { title, author, price } => {
                println!(" Fiction: \"{}\" by {}, Price: ${:.2}", title, author, price);
            }
            Magazine { title, author, price } => {
                println!(" Magazine: \"{}\" by {}, Price: ${:.2}", title, author, price);
            }
            SciFi { title, price } => {
                println!(" Sci-Fi: \"{}\", Price: ${:.2}", title, price);
            }
        }
    }
}
