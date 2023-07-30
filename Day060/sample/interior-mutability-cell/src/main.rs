use std::{fmt::{Display, Formatter, Result}, cell::Cell};

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
    sale: Cell<bool>,
}

impl Book {
    
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Title: {} | Price: {} | Sale: {}", self.title, self.price, self.sale.get())
    }
}

fn main() {
    let my_book = Book {
        author: "Shinya Yanagihara".to_string(),
        title: "Rust for Beginners".to_string(),
        category: "Rust".to_string(),
        price: 980,
        sale: Cell::new(false),
    };

    println!("{:?}", my_book);

    my_book.sale.set(true);

    println!("BookInfo:{}", my_book);

    println!("{:?}", my_book);
}
