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
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
