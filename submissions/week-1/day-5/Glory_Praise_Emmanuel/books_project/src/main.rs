enum BookGenre {
    Fiction(String),     // author
    Magazine(String),    // author
    SciFi,               // no author
}

struct Book {
    title: String,
    genre: BookGenre,
    price: u32,
}

fn main() {

    println!("\nBooks available:");

    let book1 = Book {
        title: String::from("Half Bread Is Better Than None"),
        genre: BookGenre::Fiction(String::from("Chimamanda Adichie")),
        price: 3000,
    };

    let book2 = Book {
        title: String::from("Tech Magazine - July Edition"),
        genre: BookGenre::Magazine(String::from("Web3Bridge")),
        price: 5500,
    };

    let book3 = Book {
        title: String::from("The Serious Coder"),
        genre: BookGenre::SciFi,
        price: 10500,
    };

    let books = vec![book1, book2, book3];

    for book in books {
        println!("\nTitle: {}", book.title);

        match book.genre {
            BookGenre::Fiction(author) => {
                println!("Genre: Fiction");
                println!("Author: {}", author);
            },
            BookGenre::Magazine(author) => {
                println!("Genre: Magazine");
                println!("Author: {}", author);
            },
            BookGenre::SciFi => {
                println!("Genre: Sci-Fi");
                println!("Author: (Not Available)");
            },
        }

        println!("Price: ₦{}", book.price);
    }
}
