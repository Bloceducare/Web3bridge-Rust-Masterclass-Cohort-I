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
    },
    SciFi {
        title: String,
        price: f64,
    },
}

impl Book {
    fn new_fiction(title: &str, author: &str, price: f64) -> Self {
        Book::Fiction {
            title: title.to_string(),
            author: author.to_string(),
            price,
        }
    }

    fn new_magazine(title: &str, author: &str, price: f64) -> Self {
        Book::Magazine {
            title: title.to_string(),
            author: author.to_string(),
            price,
        }
    }

    fn new_scifi(title: &str, price: f64) -> Self {
        Book::SciFi {
            title: title.to_string(),
            price,
        }
    }

    fn get_price(&self) -> f64 {
        match self {
            Book::Fiction { price, .. } => *price,
            Book::Magazine { price, .. } => *price,
            Book::SciFi { price, .. } => *price,
        }
    }

    fn get_title(&self) -> &String {
        match self {
            Book::Fiction { title, .. } => title,
            Book::Magazine { title, .. } => title,
            Book::SciFi { title, .. } => title,
        }
    }
}

fn print_book_info(book: &Book) {
    match book {
        Book::Fiction {
            title,
            author,
            price,
        } => {
            println!("Fiction: \"{}\" by {} - ${:.2}", title, author, price);
        }
        Book::Magazine {
            title,
            author,
            price,
        } => {
            println!("Magazine: \"{}\" by {} - ${:.2}", title, author, price);
        }
        Book::SciFi { title, price } => {
            println!("Sci-Fi: \"{}\" - ${:.2}", title, price);
        }
    }
}

fn main() {
    let books: Vec<Book> = vec![
        Book::new_fiction("To Kill a Mockingbird", "Harper Lee", 12.99),
        Book::new_fiction("1984", "George Orwell", 13.95),
        Book::new_magazine("National Geographic", "Various Authors", 5.99),
        Book::new_magazine("Time Magazine", "Time Inc.", 4.95),
        Book::new_scifi("Dune", 15.99),
        Book::new_scifi("The Foundation", 14.50),
        Book::new_scifi("Neuromancer", 13.25),
    ];

    println!("Welcome to the Advanced Book Library!\n");
    println!("==================================================");

    for (index, book) in books.iter().enumerate() {
        print!("{:2}. ", index + 1);
        print_book_info(book);
    }

    println!("{}", "==================================================");

    let total_value: f64 = books.iter().map(|book| book.get_price()).sum();

    println!("Total library value: ${:.2}", total_value);

    let mut fiction_count = 0;
    let mut magazine_count = 0;
    let mut scifi_count = 0;

    for book in &books {
        match book {
            Book::Fiction { .. } => fiction_count += 1,
            Book::Magazine { .. } => magazine_count += 1,
            Book::SciFi { .. } => scifi_count += 1,
        }
    }

    println!("\nLibrary Statistics:");
    println!("   Fiction books: {}", fiction_count);
    println!("   Magazines: {}", magazine_count);
    println!("   Sci-Fi books: {}", scifi_count);
    println!("   Total books: {}", books.len());

    if let Some(most_expensive) = books
        .iter()
        .max_by(|a, b| a.get_price().partial_cmp(&b.get_price()).unwrap())
    {
        println!("\nMost expensive book:");
        print!("   ");
        print_book_info(most_expensive);
    }
}
