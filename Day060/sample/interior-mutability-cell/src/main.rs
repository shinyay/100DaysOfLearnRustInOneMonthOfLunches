use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Book {
    author: String,
    title: String,
    category: String,
    price: u32,
    sale: bool,
}

impl Book {
    
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Title: {} | Price: {} | Sale: {}", self.title, self.price, self.sale)
    }
}

fn main() {
    let my_book = Book {
        author: "Shinya Yanagihara".to_string(),
        title: "Rust for Beginners".to_string(),
        category: "Rust".to_string(),
        price: 980,
        sale: false,
    };

    println!("BookInfo:{}", my_book);
}
