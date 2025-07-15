enum Book {
    Fiction { author: String },
    Magazine { author: String },
    Scifi,
}

fn main() {
    let mut books: Vec<&Book> = Vec::new();

    let fiction_book: Book = Book::Fiction {
        author: String::from("J.K Rowlings"),
    };
    let mag_book: Book = Book::Magazine {
        author: String::from("Times"),
    };

    books.push(&fiction_book);
    books.push(&mag_book);
    books.push(&Book::Scifi);

    for book in books {
        println!("====================================");
        match book {
            Book::Fiction { author } => println!("Fiction book author: {}", author),
            Book::Magazine { author } => println!("Magazine book author: {}", author),
            Book::Scifi => println!("This SciFi book has no author"),
        }
        println!("\n");
    }
}
