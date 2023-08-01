use std::{cell::RefCell, fmt::{Display, Formatter, Result}};

// #[derive(Debug)]
// struct Book {
//     author: String,
//     title: String,
//     category: String,
//     price: u32,
//     sale: bool,
// }

#[derive(Debug)]
struct Book {
    author: String,
    title: String,
    category: String,
    price: u32,
    sale: RefCell<bool>,
}

impl Book {
    
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Title: {} | Price: {}", self.title, self.price)
    }
}

fn main() {
    let my_book = Book {
        author: "Shinya Yanagihara".to_string(),
        title: "Rust for Beginners".to_string(),
        category: "Rust".to_string(),
        price: 980,
        sale: RefCell::new(false),
    };
    println!("{:?}", my_book);

    my_book.sale.replace(true);
    println!("{:?}", my_book);

    let publication_date = 2010;
    my_book.sale.replace_with(|_| if publication_date < 2010 { true } else { false });
    println!("{:?}", my_book);

}
