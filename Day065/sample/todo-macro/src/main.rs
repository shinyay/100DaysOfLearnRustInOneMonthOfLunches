struct Book {
    name: String,
    author: String,
    price: u32,
}

enum BookCategory {
    Mistery,
    Fantasy,
    Action,
    Technology,
}
 
fn register_book() -> Book {
    Book {
        name: (),
        author: (),
        price: ()
    }
}

fn get_book(book: &Book) -> Option<String> {
    todo!()
} 

fn delete_book(book: &Book) -> Result<(), String> {
    todo!()
} 

fn check_book_category(book_category: &BookCategory) {
    match book_category {
        BookCategory::Action => println!("The Category is Action"),
        BookCategory::Fantasy => println!("The Category is Fantasy"),
        BookCategory::Mistery => println!("The Category is Mistery"),
        BookCategory::Technology => println!("The Category is Technology"),
    }
}
 
fn main() {
    let book_category = BookCategory::Technology;
    check_book_category(&book_category);
}
