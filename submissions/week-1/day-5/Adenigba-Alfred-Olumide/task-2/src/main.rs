#[derive(Debug, Clone)]
enum Book {
    Fiction {
        title: String,
        author: String,
        price: f64,
    },
    Magazine {
        title: String,
        author: String,
        price: f64,
        issue_number: u32,
    },
    SciFi {
        title: String,
        price: f64,
        publication_year: u32,
        series: Option<String>,
    },
}

impl Book {
    
    fn get_price(&self) -> f64 {
        match self {
            Book::Fiction { price, .. } => *price,
            Book::Magazine { price, .. } => *price,
            Book::SciFi { price, .. } => *price,
        }
    }

    fn get_title(&self) -> &str {
        match self {
            Book::Fiction { title, .. } => title,
            Book::Magazine { title, .. } => title,
            Book::SciFi { title, .. } => title,
        }
    }

    fn get_type(&self) -> &str {
        match self {
            Book::Fiction { .. } => "Fiction",
            Book::Magazine { .. } => "Magazine",
            Book::SciFi { .. } => "Science Fiction",
        }
    }
}

fn create_books() -> Vec<Book> {
    vec![
        Book::Fiction {
            title: "Harry Potter and the Philosopher's Stone".to_string(),
            author: "J.K. Rowling".to_string(), 
            price: 5700.00,
        },
        Book::Fiction {
            title: "The Lord of the Rings".to_string(),
            author: "J.R.R. Tolkien".to_string(),
            price: 14500.00,
        },
        Book::Magazine {
            title: "Roots Magazine".to_string(),
            author: "Various Contributors".to_string(),
            price: 3000.00,
            issue_number: 245,
        },
        Book::Magazine {
            title: "Ovation Magazine".to_string(),
            author: "Editorial Team".to_string(),
            price: 2500.00,
            issue_number: 1023,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 18200.00,
            publication_year: 1965,
            series: Some("Dune Chronicles".to_string()),
        },
        Book::SciFi {
            title: "Foundation".to_string(),
            price: 6500.00,
            publication_year: 1951,
            series: Some("Foundation Series".to_string()),
        },
        Book::SciFi {
            title: "The Martian".to_string(),
            price: 5000.00,
            publication_year: 2011,
            series: None,
        },
    ]
}

// Function to print book information using pattern matching
fn print_book_info(book: &Book, index: usize) {
    print!("Book #{}: ", index + 1);
    
    match book {
        Book::Fiction { title, author, price } => {
            println!("Fiction Novel");
            println!("   Title: {}", title);
            println!("   Author: {}", author);
            println!("   Price: #{:.2}", price);
        }
        
        Book::Magazine { title, author, price, issue_number } => {
            println!("Magazine");
            println!("   Title: {}", title);
            println!("   Author: {}", author);
            println!("   Price: #{:.2}", price);
            println!("   Issue Number: #{}", issue_number);
        }
        
        Book::SciFi { title, price, publication_year, series } => {
            println!("Science Fiction");
            println!("   Title: {}", title);
            println!("   Price: #{:.2}", price);
            println!("   Publication Year: {}", publication_year);
            match series {
                Some(series_name) => println!("   Series: {}", series_name),
                None => println!("   Series: Standalone"),
            }
        }
    }
}

fn main() {
    println!("========== ADVANCED BOOK LIBRARY MANAGEMENT ==========");
    println!();
    
    // Create a vector of books
    let books = create_books();
    
    println!("BOOK CATALOG ({} books)", books.len());
    println!("--------------------------------");
    
    // pattern matching to print book's information
    for (index, book) in books.iter().enumerate() {
        print_book_info(book, index);
    }
    
    println!("--------------------------------");
    println!(" Library Operation complete!");
}