
fn main() {
    let fiction_book = Book::Fiction {
        title: String::from("The Alchemist"),
        author: String::from("Paulo Coelho"),
        price: 4000.00,
    };

    let magazine = Book::Magazine {
        title: String::from("Science Monthly"),
        author: String::from("Dr. Emeka Okafor"),
        price: 2000.00,
    };

    let sci_fi_book = Book::SciFi {
        title: String::from("Neon Horizon"),
        price: 3500.00,
    };

    let library = vec![fiction_book, magazine, sci_fi_book];

    print_book_details(library);

    println!();
}


enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

fn print_book_details(books: Vec<Book>) {
    for book in books {
        match book {
            Book::Fiction { title, author, price } => {
                println!(
                    "📘 Fiction Book: \"{}\" by {} - ₦{:.2}",
                    title, author, price
                );
            }
            Book::Magazine { title, author, price } => {
                println!(
                    "📰 Magazine: \"{}\" by {} - ₦{:.2}",
                    title, author, price
                );
            }
            Book::SciFi { title, price } => {
                println!(
                    "👽 Sci-Fi Book: \"{}\" - ₦{:.2}",
                    title, price
                );
            }
        }
    }
}

