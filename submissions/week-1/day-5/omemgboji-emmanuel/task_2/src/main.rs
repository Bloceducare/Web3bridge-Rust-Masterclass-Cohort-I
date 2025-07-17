enum Books {
    Fiction { author: String, price: f64 },
    Magazine { author: String, price: f64 },
    SciFiction { price: f64 },
}

fn main() {
    let mut list_of_books = Vec::new();

    let book1 = Books::Fiction {
        author: "Chinua Achebe".to_string(),
        price: 15.99,
    };

    let book2 = Books::Magazine {
        author: "Chimamanda Ngozi Adichie".to_string(),
        price: 5.99,
    };

    let book3 = Books::SciFiction { price: 20.00 };

    list_of_books.push(book1);
    list_of_books.push(book2);
    list_of_books.push(book3);

    for book in list_of_books {
        match book {
            Books::Fiction { author, price } => {
                println!("Fiction Book: Author: {}, Price: ${}", author, price);
            }
            Books::Magazine { author, price } => {
                println!("Magazine: Author: {}, Price: ${}", author, price);
            }
            Books::SciFiction { price } => {
                println!("SciFiction: Price: ${}", price);
            }
        }
    }
}
