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

    let author = String::from("Ayoola Olaoye");

    let fiction_book = Books {
        title: String::from("The Man Behind the Counter"),
        book_type: BooksType::Fiction(author.clone()),
        price: 300,
    };
    let magazine_book = Books {
        title: String::from("Forbes Top 10"),
        book_type: BooksType::Magazine(author.clone()),
        price: 300,
    };
    let scifi_book = Books {
        title: String::from("An Encounter from Space"),
        book_type: BooksType::SciFi,
        price: 300,
    };

    books_list.push(fiction_book);
    books_list.push(magazine_book);
    books_list.push(scifi_book);

println!("===Printing library catalogue===");
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
