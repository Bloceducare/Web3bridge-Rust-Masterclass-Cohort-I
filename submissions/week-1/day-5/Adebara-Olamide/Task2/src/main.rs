enum Books {
    Fiction(f32),
    Magazine(String, f32),
    SciFi {
        author_name: String,
        price: f32,
    },
}

fn main() {
    let magazine = Books::Magazine(String::from("John Wick"), 9.99);
    let friction = Books::Fiction(12.99);
    let scifi = Books::SciFi { author_name: String::from("Adebara Olamide"), price: 15.99 };

    let book: Vec<Books> = vec![magazine, friction, scifi];

    for item in book {
        match item {
            Books::Fiction(price) => {
                println!("This is a fiction book, costing: ${}", price);
            }
            Books::Magazine(name, price) => {
                println!("This is a magazine with the name: {}, costing ${}", name, price);
            }
            Books::SciFi { author_name, price } => {
                println!("This is a Sci-Fi book by author: {} costing ${}", author_name, price);
            }
        }
    }
}
