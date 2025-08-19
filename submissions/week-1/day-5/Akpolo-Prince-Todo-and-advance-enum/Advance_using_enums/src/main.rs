// Task 2: Advanced match using Enums
// Requirements:
// * Print out a list of books and their information
// * Books can be Fiction, Magazine, and SciFi
// * Magazine and Fiction books include the authors name
// All books include the price
// Notes:
// * Use an enum for the books with data associated with each variant
// * Create one of each book and place into a vector
// * Use a match expression while iterating the vector to print the book info





#[derive(Debug, PartialEq)]
enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

fn main() {
    let books: Vec<Book> = vec![
        Book::Fiction { title: String::from("The Great Gatsby"), author: String::from("F. Scott Fitzgerald"), price: 10.99 },
        Book::Magazine { title: String::from("National Geographic"), author: String::from("National Geographic Society"), price: 5.99 },
        Book::SciFi { title: String::from("Dune"), price: 12.99 },
    ];

    // for book in books {
    //     match book {
    //         Book::Fiction { title, author, price } => {
    //             println!("Fiction Book: {}, Author: {}, Price: ${}", title, author, price);
    //         }
    //         Book::Magazine { title, author, price } => {
    //             println!("Magazine: {}, Author: {}, Price: ${}", title, author, price);
    //         }
    //         Book::SciFi { title, price } => {
    //             println!("Sci-Fi Book: {}, Price: ${}", title, price);
    //         }
    //     }
    // }

        println!("All books have been listed.");
 
    fn print_book_info(book: &Book) {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction Book: {}, Author: {}, Price: ${}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine: {}, Author: {}, Price: ${}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("Sci-Fi Book: {}, Price: ${}", title, price);
            }
        }
    }
    fn print_all_books(books: &[Book]) {
        for book in books {
            print_book_info(book);
        }
    }
    print_all_books(&books);

}