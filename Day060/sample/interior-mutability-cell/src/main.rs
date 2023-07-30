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
        write!(f, "")
    }
}

fn main() {
    println!("Hello, world!");
}
