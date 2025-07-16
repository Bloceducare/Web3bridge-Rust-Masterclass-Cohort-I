/**

Task 2: Advanced match using Enums
Requirements:
Print out a list of books and their information
Books can be Fiction, Magazine, and SciFi
Magazine and Fiction books include the authors name
All books include the price
Notes:
Use an enum for the books with data associated with each variant
Create one of each book and place into a vector
Use a match expression while iterating the vector to print the book info

 */

enum Book {
    Fiction(String, f64),
    Magazine(String, f64),
    SciFi(String, f64),
}

fn main() {
    let books = vec![
        Book::Fiction("John Doe".to_string(), 10.0),
        Book::Magazine("Jane Doe".to_string(), 10.0),
        Book::SciFi("Jim Doe".to_string(), 10.0),
    ];

    for book in books {
        match book {
            Book::Fiction(author, price) => println!("Fiction book by {} for ${}", author, price),
            Book::Magazine(author, price) => println!("Magazine book by {} for ${}", author, price),
            Book::SciFi(author, price) => println!("SciFi book by {} for ${}", author, price),
        }
    }
}
