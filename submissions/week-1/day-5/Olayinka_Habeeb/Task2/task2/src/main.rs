#[derive(Debug)]
enum YinkaBook {
    Fiction { title: String, author: String, price: u8 },
    Magazine { title: String, author: String, price: u8 },
    SciFi { title: String, price: u8 },
}

fn print_book_info(book: &YinkaBook) {
    match book {
        YinkaBook::Fiction { title, author, price } => {
            println!("For Fiction Book: '{}' by {}, Price: ${}", title, author, price);
        }
        YinkaBook::Magazine { title, author, price } => {
            println!("For Magazine Book: '{}' by {}, Price: ${}", title, author, price);
        }
        YinkaBook::SciFi { title, price } => {
            println!("For SciFi Book: '{}', Price: ${}", title, price);
        }
    }
}

fn main() {
    let fiction = YinkaBook::Fiction {
        title: "The Rustacean Chronicles".to_string(),
        author: "Yinka Habeeb".to_string(),
        price: 12,
    };

    let magazine = YinkaBook::Magazine {
        title: "Web3bridge Rust".to_string(),
        author: "Yinka Habeeb".to_string(),
        price: 9,
    };

    let scifi = YinkaBook::SciFi {
        title: "Rust".to_string(),
        price: 10,
    };

    let books = vec![fiction, magazine, scifi];

    println!("Book List:");
    for book in &books {
        print_book_info(book);
    }
}
