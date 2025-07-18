enum Book {
    Fiction { author: String, price: f32 },
    Magazine { author: String, price: f32 },
    SciFi { price: f32 },
}

fn main() {
    let books = vec![
        Book::Fiction {
            author: "Buchi Emencheta".to_string(),
            price: 20.0,
        },
        Book::Magazine {
            author: "Time Magazine".to_string(),
            price: 15.0,
        },
        Book::SciFi { price: 25.0 },
    ];

    for book in &books {
        match book {
            Book::Fiction { author, price } => {
                println!("Fiction Book by {} costs ${}", author, price);
            }
            Book::Magazine { author, price } => {
                println!("Magazine by {} costs ${}", author, price);
            }
            Book::SciFi { price } => {
                println!("SciFi Book costs ${}", price);
            }
        }
    }
}