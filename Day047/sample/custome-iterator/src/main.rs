#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

#[derive(Debug, Clone)]
struct BookCollection(Vec<String>);

impl Library {
    fn new(name: &str) -> Self { 
        Self { 
            name: name.to_string(), 
            books: BookCollection(Vec::new()),
        }
    }

    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }

    fn get_book(&mut self) -> BookCollection {
        self.books.clone()
    }
}

// Custom Itarator Implementation
impl Iterator for BookCollection {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.pop() {
            Some(book) => {
                println!("The book is: {}", book);
                Some(book)
            }
            None => None
        }
    }
    
}

fn main() {
    let mut my_library = Library::new("Rust");
    my_library.add_book("Rust in Practice");
    my_library.add_book("Rust in Action");
    my_library.add_book("Rust for Dummy");


    println!("My Library: {my_library:?}");
}
