#[derive(Debug)]
enum BookType {
    Fiction,
    Magazine,
    SciFi,
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    book_type: BookType
}

impl Book {
    fn new(title: String, author: String, book_type: BookType) -> Book {
        Book { title, author, book_type}
    }
}

fn main() {
    let mut books_list: Vec<Book> = Vec::new();

    let book1 = Book::new(
        String::from("Half of a yellow Sun"),
        String::from("Chimamanda Ngozi Adichie"),
        BookType::Fiction,
    );

    books_list.push(book1);

    let book2 = Book::new(
        String::from("Forbes Top 100"),
        String::from("Forbes"),
        BookType::Magazine,
    );

    books_list.push(book2);

    let book3 = Book::new(
        String::from("The Fifth Season"),
        String::from("N.K. Jemisin"),
        BookType::SciFi,
    );

    books_list.push(book3);

    for book in books_list {
        match book.book_type {
            BookType::Fiction => println!("{} by {} ", book.title, book.author),
            BookType::Magazine => println!("{} ", book.title),
            BookType::SciFi => println!("{} by {} ", book.title, book.author),
        }
    }
}
