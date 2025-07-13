#[derive(Debug)]
struct Fiction {
    name: String,
    author: String,
    price: f64,
}

#[derive(Debug)]
enum BookType {
    Fiction(Fiction),
    SciFi { author: String, price: f64 },
    Magazine(String, f64),
}

fn main() {
    let books = vec![
        BookType::Fiction(Fiction {
            name: "Jungle Man".to_string(),
            author: "Josh".to_string(),
            price: 23.54,
        }),
        BookType::SciFi {
            author: "Dean".to_string(),
            price: 24.99,
        },
        BookType::Magazine("National Geographic Team".to_string(), 12.99),
    ];

    for (index, book) in books.iter().enumerate() {
        print!("Book {}: ", index + 1);

        match book {
            BookType::Fiction(Fiction { name, author, price }) => {
                println!("{:#?}", book);
            }
            BookType::SciFi { author, price } => {
                println!("{:#?}", book);
            }
            BookType::Magazine(author, price) => {
                println!("{:#?}", book);
            }
        }
    }
}