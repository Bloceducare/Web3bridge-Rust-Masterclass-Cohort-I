

fn main() {

    let mut book_list: Vec<&Books> = Vec::new();
    let book1 = Books::Fiction(100, String::from("john doe"));
    let book2 = Books::Magazine(300, String::from("Aare"));
    let book3 = Books::SciFi(89);

    book_list.push(&book1);
    book_list.push(&book2);
    book_list.push(&book3);

    book_list.iter().for_each(|book| {
        match book {
            Books::Magazine(price, author) => println!("Magazine: price is {}, author is {}", price, author),
            Books::Fiction(price, author) => println!("Fiction: price is {}, author is {}", price, author),
            Books::SciFi(price) => println!("Sci fi: price is {}", price),
        }
    });
}




#[derive(Debug, PartialEq)]
enum Books {
    Fiction(u32, String),
    Magazine(u32, String),
    SciFi(u32),
}