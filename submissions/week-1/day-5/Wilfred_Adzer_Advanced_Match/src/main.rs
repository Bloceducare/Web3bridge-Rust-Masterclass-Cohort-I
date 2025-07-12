#[derive(Debug)]
enum Book {
    Fiction { title: String, author: String, price: f32 },
    Magazine { title: String, author: String, price: f32 },
    SciFi { title: String, price: f32 },
}

fn print_books(books: &Vec<Book>) {
    for book in books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction Book:\n  Title: {}\n  Author: {}\n  Price: ${:.2}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine:\n  Title: {}\n  Author: {}\n  Price: ${:.2}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!(" SciFi Book:\n  Title: {}\n  Price: ${:.2}", title, price);
            }
        }
        println!("---");
    }
}

fn main() {
    let book1 = Book::Fiction {
        title: "The Great Gatsby".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
        price: 15.99,
    };

    let book2 = Book::Magazine {
        title: "National Geographic".to_string(),
        author: "Various".to_string(),
        price: 6.99,
    };

    let book3 = Book::SciFi {
        title: "Dune".to_string(),
        price: 12.49,
    };

    let books = vec![book1, book2, book3];

    print_books(&books);
}
