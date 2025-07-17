enum BookType {
    Fiction,
    Magazine,
    SciFi,
}

struct Book {
    title: String,
    author: String,
    year: i32,
    description: String,
    book_type: BookType,
}

impl Book {
    fn new(title: String, author: String, year: i32, description: String, book_type: BookType) -> Book {
        Book { title, author, year, description, book_type }
    }
}


fn main() {
    let book1 = Book::new(String::from("Half of a Yellow Sun"), String::from("Chimamanda Ngozi Adichie"), 2006, String::from("A novel about the Nigerian Civil War"), BookType::Fiction);
    let book2 = Book::new(String::from("Binti: Home "), String::from("Nnedi Okorafor"), 2015, String::from("A story about Binti, a young Himba woman from Earth"), BookType::Fiction);
    let book3 = Book::new(String::from("The Fifth Season"), String::from("N.K. Jemisin"), 2015, String::from("A story about a woman who is the fifth season"), BookType::SciFi);
    let book4 = Book::new(String::from("Things Fall Apart"), String::from("Chinua Achebe"), 1958, String::from("A story about a man who is the first son of his father"), BookType::Magazine);
    let book5 = Book::new(String::from("Purple Hibiscus"), String::from("Chimamanda Ngozi Adichie"), 2003, String::from("A story about a woman who is the fifth season"), BookType::Fiction);

    let books: Vec<Book> = vec![book1, book2, book3, book4, book5];

    for book in books {
        match book.book_type {
            BookType::Fiction => println!("{} by {} - {} - {}", book.title, book.author, book.year, book.description),
            BookType::Magazine => println!("{} - {} - {}", book.title, book.year, book.description),
            BookType::SciFi => println!("{} by {} - {} - {}", book.title, book.author, book.year, book.description),
        }
    }
}
