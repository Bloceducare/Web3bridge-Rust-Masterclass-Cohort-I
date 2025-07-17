pub enum Books {
    Fiction { author: String, price: f64 },
    Magazine { author: String, price: f64 },
    SciFi { price: f64 },
}

pub struct Library {
    books: Vec<Books>,
}

impl Library {
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    pub fn new_fiction(&mut self, author: &str, price: f64) {
        self.books.push(Books::Fiction {
            author: author.to_string(),
            price,
        });
    }

    pub fn new_magazine(&mut self, author: &str, price: f64) {
        self.books.push(Books::Magazine {
            author: author.to_string(),
            price,
        });
    }

    pub fn new_scifi(&mut self, price: f64) {
        self.books.push(Books::SciFi { price });
    }

    pub fn print_books_details(&self) {
        for book in &self.books {
            match book {
                Books::Fiction { author, price } => {
                    println!("Fiction book by {} costs ₦{:.2}", author, price);
                }
                Books::Magazine { author, price } => {
                    println!("Magazine by {} costs ₦{:.2}", author, price);
                }
                Books::SciFi { price } => {
                    println!("SciFi book costs ₦{:.2}", price);
                }
            }
        }
    }
}

fn main() {
    let mut library = Library::new();

    library.new_fiction("Abdul Jamal", 750.00);
    library.new_magazine("Tribune", 250.00);
    library.new_scifi(350.00);

    library.print_books_details();
}
