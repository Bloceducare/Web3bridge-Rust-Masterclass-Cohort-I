fn main() {
    let books = vec![
        BookGenre::Fiction {
            title: String::from("The Great Gatsby"),
            author: String::from("F. Scott Fitzgerald"),
            price: 10.99,
        },
        BookGenre::Magazine {
            title: String::from("National Geographic"),
            author: String::from("Various"),
            price: 5.99,
        },
        BookGenre::NonFiction {
            title: String::from("Sapiens: A Brief History of Humankind"),
            author: String::from("Yuval Noah Harari"),
            price: 15.99,
        },
        BookGenre::Science {
            title: String::from("A Brief History of Time"),
            price: 12.99,
        },
        BookGenre::History {
            title: String::from("The History of the Ancient World"),
            price: 14.99,
        },
    ];

    println!("Book Details:");
    println!("===========================================================");
    print_book_details(books);
    println!("===========================================================");    
}
enum BookGenre {
    Fiction {
        title: String,
        author: String,
        price: f64,
    },
    Magazine {
        title: String,
        author: String,
        price: f64,
    },
    NonFiction {
        title: String,
        author: String,
        price: f64,
    },
    Science {
        title: String,
        price: f64,
    },
    History {
        title: String,
        price: f64,
    },
}

fn print_book_details(books: Vec<BookGenre>) {
    for book in books {
        match book {
            BookGenre::Fiction { title, author, price } => {
                println!("Fiction Book: \"{}\" by {} - ${:.2}", title, author, price);
            }
            BookGenre::Magazine { title, author, price } => {
                println!("Magazine: \"{}\" by {} - ${:.2}", title, author, price);
            }
            BookGenre::NonFiction { title, author, price } => {
                println!("Non-Fiction Book: \"{}\" by {} - ${:.2}", title, author, price);
            }
            BookGenre::Science { title, price } => {
                println!("Science Book: \"{}\" - ${:.2}", title, price);
            }
            BookGenre::History { title, price } => {
                println!("History Book: \"{}\" - ${:.2}", title, price);
            }
        }
    }
}