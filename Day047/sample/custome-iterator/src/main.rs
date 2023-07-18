#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

#[derive(Debug)]
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
}

fn main() {
    let my_library = Library::new("Rust");
    println!("My Library: {my_library:?}");
}
