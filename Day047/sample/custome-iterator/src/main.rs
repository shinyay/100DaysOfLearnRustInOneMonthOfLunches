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

fn main() {
    let my_library = Library::new("Rust");
    println!("My Library: {my_library:?}");
}
