#[derive(Debug)]
enum BooksType {
    Fiction(String),
    Magazine(String),
    SciFi,
}

#[derive(Debug)]
struct Books {
    title: String,
    book_type: BooksType,
    price: u32,
}

fn main() {
    let mut books_list: Vec<Books> = Vec::new();

    let author = String::from("Ekezie Uchenna");

  
    let magazine_book = Books {
        title: String::from("Rust Magazine"),
        book_type: BooksType::Magazine(author.clone()),
        price: 100,
    };

    let fiction_book = Books {
        title: String::from("The Rust Chronicles"),
        book_type: BooksType::Fiction(author.clone()),
        price: 100,
    };
    let scifi_book = Books {
        title: String::from("The Rust Ownership Guide"),
        book_type: BooksType::SciFi,
        price: 100,
    };

    books_list.push(magazine_book);
    books_list.push(fiction_book);
    books_list.push(scifi_book);

println!("Books List:");
    for (i, el) in books_list.iter().enumerate() {
        match el.book_type {
            BooksType::Fiction(_) => {
                println!("Fiction: {:#?}", el)
            }
            BooksType::Magazine(_) => {
                println!("Magazine: {:#?}", el)
            }
            BooksType::SciFi => {
                println!("SciFi: {:#?}", el)
            }
        }
    }
}